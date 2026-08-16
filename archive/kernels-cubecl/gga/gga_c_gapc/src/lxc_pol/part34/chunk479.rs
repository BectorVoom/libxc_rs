//! GGA_C_GAPC lxc pol — lxc_pol part 34 (v4rho2sigma2_13) CSE chunk 479/1427 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part34_v4rho2sigma2_13_chunk479<F: Float>(t818: F, t329: F, t332: F, t197: F, t1764: F, t333: F, t2555: F, t2560: F, t2563: F, t2568: F, t2571: F, t2573: F, t2578: F, t2581: F, t2585: F, t2588: F, t2591: F, t2594: F, t2600: F, t2603: F, t2607: F, t2611: F, t2616: F, t2620: F, t2622: F, t2625: F, t2629: F, t2632: F, t2635: F, t2639: F, t321: F, t326: F, t788: F, t890: F, t904: F, t929: F, t949: F, t957: F) -> (F, F, F, F) {
    let t2642 = t818 * t818;
    let t2643 = t2642 * t329;
    let t2644 = t2643 * t332;
    let t2645 = t197 * t2644;
    let t2648 = t1764 * t333;
    let t2653 = F::cast_from(0.67632724766374884053e-5_f64) * t957 * t2555 + F::cast_from(0.687148483626368822e-7_f64) * t2560 * t2563 - F::cast_from(0.91631250291576282414e-7_f64) * t2568 * t2563 + F::cast_from(0.33816362383187442026e-5_f64) * t2571 * t2573 + F::cast_from(0.59127296360574214771e-4_f64) * t2578 * t2581 + F::cast_from(0.12357942809624928455e-3_f64) * t904 * t2585 + F::cast_from(0.69504740211613770836e-4_f64) * t2588 * t788 + F::cast_from(0.2318836277704281739e-4_f64) * t2591 * t2594 - F::cast_from(0.2318836277704281739e-4_f64) * t2591 * t2600 + F::cast_from(0.6487109086417285278e-2_f64) * t321 * t2603 - F::cast_from(0.27801896084645508334e-2_f64) * t321 * t2607 - F::cast_from(0.27801896084645508334e-2_f64) * t890 * t2611 - F::cast_from(0.13900948042322754167e-2_f64) * t890 * t2616 - F::cast_from(0.40544431790108032986e-3_f64) * t2620 * t2622 - F::cast_from(0.19323635647535681158e-6_f64) * t2625 * t2629 + F::cast_from(0.343574241813184411e-6_f64) * t2632 * t2629 + F::cast_from(0.38647271295071362317e-6_f64) * t2635 * t2639 - F::cast_from(0.11594181388521408695e-4_f64) * t326 * t2645 - F::cast_from(0.15330084280378307052e-3_f64) * t326 * t2648 + F::cast_from(0.54106179813099907242e-4_f64) * t929 * t949;
    (t2642, t2645, t2648, t2653)
}
