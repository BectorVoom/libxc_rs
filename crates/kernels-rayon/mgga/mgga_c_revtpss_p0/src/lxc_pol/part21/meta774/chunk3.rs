//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2753/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2753(t10638: f64, t2723: f64, t10943: f64, t14671: f64, t14686: f64, t14931: f64, t10627: f64, t10861: f64, t14676: f64, t14691: f64, t14785: f64, t231: f64, t2646: f64, t2745: f64, t2747: f64, t2749: f64, t40581: f64, t40586: f64, t40594: f64, t40600: f64, t40607: f64, t40611: f64, t40673: f64, t4362: f64, t4364: f64, t4365: f64, t50423: f64, t50628: f64, t50632: f64, t50634: f64, t50643: f64, t50649: f64) -> (f64, f64) {
    let t50666 = t2723 * t10638;
    let t50673 = t14931 * t14686 * t14671 * t10943;
    let t50675 = 0.25724410870841842183e-2_f64 * t2745 * t2747 * t14691 * t2646 + 0.30492001685571196935e-3_f64 * t50628 - 0.76230004213927992337e-4_f64 * t50632 + 0.68026775414003982662e-1_f64 * t50634 + 0.25724410870841842183e-1_f64 * t2745 * t40673 * t4365 * t231 * t10627 - 0.38115002106963996168e-4_f64 * t50643 + 0.12862205435420921092e-2_f64 * t4362 * t4364 * t14676 * t10943 - 0.25724410870841842183e-1_f64 * t2745 * t14785 * t50649 * t2749 + 0.30011812682648815881e-2_f64 * t4362 * t4364 * t4365 * t10861 + 0.76230004213927992336e-4_f64 * t40581 + 0.15246000842785598467e-3_f64 * t40586 + 0.13605355082800796533e0_f64 * t40594 + 0.30492001685571196935e-4_f64 * t40600 + t40607 - t40611 - 0.12862205435420921092e-1_f64 * t2745 * t14785 * t50423 * t2749 + 0.42874018118069736972e-3_f64 * t4362 * t4364 * t4365 * t50666 + 0.76230004213927992338e-4_f64 * t50673;
    (t50666, t50675)
}
