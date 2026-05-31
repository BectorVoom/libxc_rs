//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 1440/2270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1440<F: Float>(t5701: F, t9962: F, t13778: F, t13779: F, t13781: F, t13786: F, t13793: F, t13797: F, t13798: F, t13801: F, t13804: F, t13807: F, t13810: F, t3934: F, t5671: F, t9735: F) -> F {
    let t13813 = F::cast_from(0.20007875121765877254e-2_f64) * t9962 * t5701;
    let t13814 = t13778 - F::cast_from(0.76220476654346199061e-4_f64) * t13779 - F::cast_from(0.22675591804667994221e-1_f64) * t13781 - F::cast_from(0.85748036236139473944e-2_f64) * t3934 * t13786 - t9735 - F::cast_from(0.34299214494455789578e-2_f64) * t5671 * t13793 + t13797 - F::cast_from(35.0_f64) / F::cast_from(216.0_f64) * t13798 + F::cast_from(0.10164000561857065645e-4_f64) * t13801 - F::cast_from(0.12862205435420921092e-2_f64) * t13804 * t13807 - F::cast_from(0.80031500487063509015e-2_f64) * t13810 + t13813;
    t13814
}
