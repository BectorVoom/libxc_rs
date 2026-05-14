//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 1057/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk1057<F: Float>(t27451: F, t27469: F, t27685: F, t27944: F, t27962: F, t27992: F, t28013: F, t28031: F, t1610: F, t8432: F, t1620: F, t8436: F, t2347: F, t6638: F, t8455: F, t27068: F, t27071: F, t27074: F, t27076: F, t27079: F, t27082: F, t27084: F, t27087: F, t27090: F, t27093: F, t27096: F, t27099: F, t27102: F, t27105: F, t27107: F, t27109: F, t27111: F, t27113: F) -> (F, F, F, F, F, F) {
    let t28034 = t27451 + t27469 + t27685 + t27944 + t27962 + t27992 + t28013 + t28031;
    let t28036 = t8432 * t1610;
    let t28046 = t8436 * t1620;
    let t28049 = t2347 * t6638;
    let t28053 = t8455 * t1620;
    let t28074 = -0.125e0 * t27068 - 0.41666666666666666667e-1 * t27071 + 0.60703125e-1 * t27074 + 0.26979166666666666667e-1 * t27076 + 0.375e0 * t27079 - 0.89930555555555555553e-2 * t27082 + 0.26979166666666666666e-1 * t27084 + 0.29976851851851851851e-2 * t27087 + 0.44965277777777777777e-2 * t27090 - 0.625e-1 * t27093 - 0.4046875e-1 * t27096 + 0.375e0 * t27099 + 0.13489583333333333333e-1 * t27102 - 0.20833333333333333333e-1 * t27105 - 0.125e0 * t27107 - 0.125e0 * t27109 + 0.101171875e-1 * t27111 - 0.10791666666666666667e0 * t27113;
    (t28034, t28036, t28046, t28049, t28053, t28074)
}
