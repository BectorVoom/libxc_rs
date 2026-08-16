//! GGA_C_GAPC lxc pol — lxc_pol part 38 (v4rho2sigma2_17) CSE chunk 459/1307 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part38_v4rho2sigma2_17_chunk459(t2546: f64, t818: f64, t2545: f64, t2474: f64, t2477: f64, t2482: f64, t2483: f64, t2486: f64, t2489: f64, t2494: f64, t2498: f64, t2501: f64, t2505: f64, t2508: f64, t2511: f64, t2515: f64, t2521: f64, t2524: f64, t2526: f64, t2529: f64, t2532: f64, t2535: f64, t2537: f64, t2542: f64, t321: f64, t322: f64, t334: f64, t877: f64, t886: f64, t898: f64, t899: f64, t907: f64, t913: f64, t917: f64, t920: f64, t957: f64, t962: f64) -> (f64, f64, f64) {
    let t2547 = t2546 * t818;
    let t2548 = t2545 * t2547;
    let t2551 = 0.2085142206348413125e-3_f64 * t2474 * t899 - 0.33816362383187442026e-5_f64 * t2477 * t962 - 0.96618178237678405792e-8_f64 * t2482 * t2483 + 0.21417029509352046616e-4_f64 * t957 * t2486 + 0.13900948042322754167e-2_f64 * t321 * t2489 + 0.40544431790108032986e-3_f64 * t917 * t2494 - 0.57970906942607043474e-5_f64 * t2498 * t334 - 0.12357942809624928455e-3_f64 * t2501 * t2505 - 0.6487109086417285278e-2_f64 * t321 * t2508 - 0.16217772716043213195e-2_f64 * t917 * t2511 + 0.13900948042322754167e-2_f64 * t2515 * t322 + 0.27801896084645508334e-2_f64 * t886 * t913 + 0.40544431790108032986e-3_f64 * t2521 * t920 - 0.12357942809624928455e-3_f64 * t2524 * t2526 - 0.41193142698749761516e-5_f64 * t2529 * t2532 - 0.33787026491756694155e-5_f64 * t2535 * t2537 + 0.69504740211613770836e-4_f64 * t898 * t877 + 0.12357942809624928455e-3_f64 * t2542 * t907 - 0.67632724766374884053e-5_f64 * t957 * t2548;
    (t2547, t2548, t2551)
}
