//! GGA_C_GAPC lxc pol — lxc_pol part 21 (v4rho2sigma2_0) CSE chunk 475/1307 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part21_v4rho2sigma2_0_chunk475<F: Float>(t818: F, t329: F, t332: F, t197: F, t1764: F, t333: F, t2555: F, t2560: F, t2563: F, t2568: F, t2571: F, t2573: F, t2578: F, t2581: F, t2585: F, t2588: F, t2591: F, t2594: F, t2600: F, t2603: F, t2607: F, t2611: F, t2616: F, t2620: F, t2622: F, t2625: F, t2629: F, t2632: F, t2635: F, t2639: F, t321: F, t326: F, t788: F, t890: F, t904: F, t929: F, t949: F, t957: F) -> (F, F, F, F) {
    let t2642 = t818 * t818;
    let t2643 = t2642 * t329;
    let t2644 = t2643 * t332;
    let t2645 = t197 * t2644;
    let t2648 = t1764 * t333;
    let t2653 = F::new(0.67632724766374884053e-5) * t957 * t2555 + F::new(0.687148483626368822e-7) * t2560 * t2563 - F::new(0.91631250291576282414e-7) * t2568 * t2563 + F::new(0.33816362383187442026e-5) * t2571 * t2573 + F::new(0.59127296360574214771e-4) * t2578 * t2581 + F::new(0.12357942809624928455e-3) * t904 * t2585 + F::new(0.69504740211613770836e-4) * t2588 * t788 + F::new(0.2318836277704281739e-4) * t2591 * t2594 - F::new(0.2318836277704281739e-4) * t2591 * t2600 + F::new(0.6487109086417285278e-2) * t321 * t2603 - F::new(0.27801896084645508334e-2) * t321 * t2607 - F::new(0.27801896084645508334e-2) * t890 * t2611 - F::new(0.13900948042322754167e-2) * t890 * t2616 - F::new(0.40544431790108032986e-3) * t2620 * t2622 - F::new(0.19323635647535681158e-6) * t2625 * t2629 + F::new(0.343574241813184411e-6) * t2632 * t2629 + F::new(0.38647271295071362317e-6) * t2635 * t2639 - F::new(0.11594181388521408695e-4) * t326 * t2645 - F::new(0.15330084280378307052e-3) * t326 * t2648 + F::new(0.54106179813099907242e-4) * t929 * t949;
    (t2642, t2645, t2648, t2653)
}
