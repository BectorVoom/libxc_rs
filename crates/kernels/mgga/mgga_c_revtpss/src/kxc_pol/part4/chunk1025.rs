//! MGGA_C_REVTPSS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1025/1274 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part4_v3rho3_1_chunk1025<F: Float>(t13805: F, t5673: F, t5674: F, t5697: F, t9962: F, t5701: F, t13778: F, t13779: F, t13781: F, t13786: F, t13793: F, t13797: F, t13798: F, t13801: F, t13804: F, t3934: F, t5671: F, t9735: F) -> (F,) {
    let t13807 = t5673 * t5674 * t13805;
    let t13810 = t9962 * t5697;
    let t13813 = 0.20007875121765877254e-2 * t9962 * t5701;
    let t13814 = t13778 - 0.76220476654346199061e-4 * t13779 - 0.22675591804667994221e-1 * t13781 - 0.85748036236139473944e-2 * t3934 * t13786 - t9735 - 0.34299214494455789578e-2 * t5671 * t13793 + t13797 - 35.0 / 216.0 * t13798 + 0.10164000561857065645e-4 * t13801 - 0.12862205435420921092e-2 * t13804 * t13807 - 0.80031500487063509015e-2 * t13810 + t13813;
    (t13814,)
}
