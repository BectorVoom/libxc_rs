//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta401 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1488;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1489;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1490;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta401<F: Float>(t11643: F, t11994: F, t12025: F, t3127: F, t3172: F, t3105: F, t3196: F, t11656: F, t2852: F, t3154: F, t2251: F, t11648: F, t3124: F, t1041: F, t11622: F, t12021: F, t3173: F, t1032: F, t1040: F, t11902: F, t11762: F, t3241: F, t1047: F, t11659: F, t11703: F, t11705: F, t11714: F, t11883: F, t3177: F, t3238: F, t3248: F, t3255: F, t4892: F, t4899: F, t11752: F, t11755: F, t1011: F, t3247: F, t697: F, t3254: F, t11789: F, t11937: F, t225: F, t42051: F, t366: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t42190, t42193, t42195, t42204, t42216, t42227) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1488::<F>(t11643, t11994, t12025, t3127, t3172, t3105, t3196, t11656, t2852, t3154, t2251, t11648, t3124);
        let t42246 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1489::<F>(t1041, t11622, t3172, t12021, t3173, t1032, t1040, t11902, t11762, t3241, t1047, t11659, t11703, t11705, t11714, t11883, t3177, t3238, t3248, t3255, t42216, t42227, t4892, t4899);
        let (t42249, t42251, t42254, t42257, t42259, t42261, t42262) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1490::<F>(t11752, t3241, t11755, t1011, t3247, t697, t3254, t11789, t11937, t225, t42051, t366);
    (t42190, t42193, t42195, t42204, t42246, t42249, t42251, t42254, t42257, t42259, t42261, t42262)
}
