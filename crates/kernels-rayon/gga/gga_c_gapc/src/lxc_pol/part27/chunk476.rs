//! GGA_C_GAPC lxc pol — lxc_pol part 27 (v4rho2sigma2_6) CSE chunk 476/1310 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part27_v4rho2sigma2_6_chunk476(t818: f64, t329: f64, t332: f64, t197: f64, t1764: f64, t333: f64, t2555: f64, t2560: f64, t2563: f64, t2568: f64, t2571: f64, t2573: f64, t2578: f64, t2581: f64, t2585: f64, t2588: f64, t2591: f64, t2594: f64, t2600: f64, t2603: f64, t2607: f64, t2611: f64, t2616: f64, t2620: f64, t2622: f64, t2625: f64, t2629: f64, t2632: f64, t2635: f64, t2639: f64, t321: f64, t326: f64, t788: f64, t890: f64, t904: f64, t929: f64, t949: f64, t957: f64) -> (f64, f64, f64, f64) {
    let t2642 = t818 * t818;
    let t2643 = t2642 * t329;
    let t2644 = t2643 * t332;
    let t2645 = t197 * t2644;
    let t2648 = t1764 * t333;
    let t2653 = 0.67632724766374884053e-5_f64 * t957 * t2555 + 0.687148483626368822e-7_f64 * t2560 * t2563 - 0.91631250291576282414e-7_f64 * t2568 * t2563 + 0.33816362383187442026e-5_f64 * t2571 * t2573 + 0.59127296360574214771e-4_f64 * t2578 * t2581 + 0.12357942809624928455e-3_f64 * t904 * t2585 + 0.69504740211613770836e-4_f64 * t2588 * t788 + 0.2318836277704281739e-4_f64 * t2591 * t2594 - 0.2318836277704281739e-4_f64 * t2591 * t2600 + 0.6487109086417285278e-2_f64 * t321 * t2603 - 0.27801896084645508334e-2_f64 * t321 * t2607 - 0.27801896084645508334e-2_f64 * t890 * t2611 - 0.13900948042322754167e-2_f64 * t890 * t2616 - 0.40544431790108032986e-3_f64 * t2620 * t2622 - 0.19323635647535681158e-6_f64 * t2625 * t2629 + 0.343574241813184411e-6_f64 * t2632 * t2629 + 0.38647271295071362317e-6_f64 * t2635 * t2639 - 0.11594181388521408695e-4_f64 * t326 * t2645 - 0.15330084280378307052e-3_f64 * t326 * t2648 + 0.54106179813099907242e-4_f64 * t929 * t949;
    (t2642, t2645, t2648, t2653)
}
