//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3257/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3257<F: Float>(t13783: F, t13804: F, t1410: F, t1868: F, t1883: F, t21969: F, t22016: F, t22279: F, t3934: F, t4012: F, t48509: F, t48516: F, t48518: F, t48529: F, t48532: F, t48563: F, t5591: F, t5673: F, t6816: F, t73847: F, t74232: F, t74249: F, t74257: F, t828: F, t85553: F, t85741: F, t85752: F, t85764: F, t85778: F) -> F {
    let t85780 = -F::cast_from(0.85748036236139473942e-4_f64) * t85741 - t48509 + F::cast_from(0.91464571985215438873e-3_f64) * t48516 + F::cast_from(0.11337795902333997111e0_f64) * t48518 + F::cast_from(0.91464571985215438872e-3_f64) * t48529 - t48532 - F::cast_from(0.22869001264178397701e-3_f64) * t74232 - F::cast_from(0.77173232612525526552e-2_f64) * t13804 * t5673 * t85553 * t22016 - F::cast_from(0.38115002106963996168e-4_f64) * t74249 + F::cast_from(0.24009450146119052705e-1_f64) * t85752 - F::cast_from(0.64311027177104605458e-3_f64) * t3934 * t5673 * t73847 * t1883 + F::cast_from(0.30492001685571196935e-3_f64) * t74257 - F::cast_from(0.60246173160355784831e-6_f64) * t48563 - F::cast_from(0.25724410870841842184e-1_f64) * t3934 * t13783 * t1883 * t22279 - F::cast_from(0.60023625365297631763e-1_f64) * t85764 + F::cast_from(0.12862205435420921092e-1_f64) * t1410 * t4012 * t828 * t21969 * t1868 + F::cast_from(0.12862205435420921092e-1_f64) * t1410 * t4012 * t828 * t6816 * t5591 + F::cast_from(0.7623000421392799234e-3_f64) * t85778;
    t85780
}
