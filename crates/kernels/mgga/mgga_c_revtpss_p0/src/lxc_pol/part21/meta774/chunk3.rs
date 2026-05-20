//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2753/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2753<F: Float>(t10638: F, t2723: F, t10943: F, t14671: F, t14686: F, t14931: F, t10627: F, t10861: F, t14676: F, t14691: F, t14785: F, t231: F, t2646: F, t2745: F, t2747: F, t2749: F, t40581: F, t40586: F, t40594: F, t40600: F, t40607: F, t40611: F, t40673: F, t4362: F, t4364: F, t4365: F, t50423: F, t50628: F, t50632: F, t50634: F, t50643: F, t50649: F) -> (F, F) {
    let t50666 = t2723 * t10638;
    let t50673 = t14931 * t14686 * t14671 * t10943;
    let t50675 = F::cast_from(0.25724410870841842183e-2_f64) * t2745 * t2747 * t14691 * t2646 + F::cast_from(0.30492001685571196935e-3_f64) * t50628 - F::cast_from(0.76230004213927992337e-4_f64) * t50632 + F::cast_from(0.68026775414003982662e-1_f64) * t50634 + F::cast_from(0.25724410870841842183e-1_f64) * t2745 * t40673 * t4365 * t231 * t10627 - F::cast_from(0.38115002106963996168e-4_f64) * t50643 + F::cast_from(0.12862205435420921092e-2_f64) * t4362 * t4364 * t14676 * t10943 - F::cast_from(0.25724410870841842183e-1_f64) * t2745 * t14785 * t50649 * t2749 + F::cast_from(0.30011812682648815881e-2_f64) * t4362 * t4364 * t4365 * t10861 + F::cast_from(0.76230004213927992336e-4_f64) * t40581 + F::cast_from(0.15246000842785598467e-3_f64) * t40586 + F::cast_from(0.13605355082800796533e0_f64) * t40594 + F::cast_from(0.30492001685571196935e-4_f64) * t40600 + t40607 - t40611 - F::cast_from(0.12862205435420921092e-1_f64) * t2745 * t14785 * t50423 * t2749 + F::cast_from(0.42874018118069736972e-3_f64) * t4362 * t4364 * t4365 * t50666 + F::cast_from(0.76230004213927992338e-4_f64) * t50673;
    (t50666, t50675)
}
