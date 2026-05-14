//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 1176/1268 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk1176<F: Float>(t3005: F, t7419: F, t9800: F, t11072: F, t11080: F, t11084: F, t2194: F, t28829: F, t28834: F, t28836: F, t28839: F, t28841: F, t28851: F, t28854: F, t28859: F, t32313: F, t3480: F, t3507: F, t4598: F, t568: F, t6018: F, t784: F, t808: F, t813: F, t833: F, t836: F) -> (F,) {
    let t33731 = t9800 * t3005 * t7419;
    let t33732 = 0.36425779656224712192e1 * t33731;
    let t33752 = t28829 + t28834 - t28836 + t28839 + t28841 - t28851 - t28854 + t33732 + t28859 + 0.23005755572352449806e1 * t833 * t568 * t836 * t32313 - 0.23005755572352449806e1 * t813 * t568 * t808 * t32313 + 0.1022478025437886658e1 * t833 * t4598 * t3507 - 0.35750489951850426669e0 * t6018 * t3480 + 0.47667319935800568892e0 * t11080 * t784 + 0.47667319935800568892e0 * t11084 * t784 - 0.61348681526273199482e1 * t2194 * t11072;
    (t33752,)
}
