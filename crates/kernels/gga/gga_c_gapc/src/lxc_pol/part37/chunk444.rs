//! GGA_C_GAPC lxc pol — lxc_pol part 37 (v4rho2sigma2_16) CSE chunk 444/1225 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part37_v4rho2sigma2_16_chunk444<F: Float>(t2546: F, t818: F, t2545: F, t2474: F, t2477: F, t2482: F, t2483: F, t2486: F, t2489: F, t2494: F, t2498: F, t2501: F, t2505: F, t2508: F, t2511: F, t2515: F, t2521: F, t2524: F, t2526: F, t2529: F, t2532: F, t2535: F, t2537: F, t2542: F, t321: F, t322: F, t334: F, t877: F, t886: F, t898: F, t899: F, t907: F, t913: F, t917: F, t920: F, t957: F, t962: F) -> (F, F, F) {
    let t2547 = t2546 * t818;
    let t2548 = t2545 * t2547;
    let t2551 = 0.2085142206348413125e-3 * t2474 * t899 - 0.33816362383187442026e-5 * t2477 * t962 - 0.96618178237678405792e-8 * t2482 * t2483 + 0.21417029509352046616e-4 * t957 * t2486 + 0.13900948042322754167e-2 * t321 * t2489 + 0.40544431790108032986e-3 * t917 * t2494 - 0.57970906942607043474e-5 * t2498 * t334 - 0.12357942809624928455e-3 * t2501 * t2505 - 0.6487109086417285278e-2 * t321 * t2508 - 0.16217772716043213195e-2 * t917 * t2511 + 0.13900948042322754167e-2 * t2515 * t322 + 0.27801896084645508334e-2 * t886 * t913 + 0.40544431790108032986e-3 * t2521 * t920 - 0.12357942809624928455e-3 * t2524 * t2526 - 0.41193142698749761516e-5 * t2529 * t2532 - 0.33787026491756694155e-5 * t2535 * t2537 + 0.69504740211613770836e-4 * t898 * t877 + 0.12357942809624928455e-3 * t2542 * t907 - 0.67632724766374884053e-5 * t957 * t2548;
    (t2547, t2548, t2551)
}
