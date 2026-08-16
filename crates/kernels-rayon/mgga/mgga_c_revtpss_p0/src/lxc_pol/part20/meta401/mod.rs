//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta401 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1488;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1489;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1490;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta401(t11643: f64, t11994: f64, t12025: f64, t3127: f64, t3172: f64, t3105: f64, t3196: f64, t11656: f64, t2852: f64, t3154: f64, t2251: f64, t11648: f64, t3124: f64, t1041: f64, t11622: f64, t12021: f64, t3173: f64, t1032: f64, t1040: f64, t11902: f64, t11762: f64, t3241: f64, t1047: f64, t11659: f64, t11703: f64, t11705: f64, t11714: f64, t11883: f64, t3177: f64, t3238: f64, t3248: f64, t3255: f64, t4892: f64, t4899: f64, t11752: f64, t11755: f64, t1011: f64, t3247: f64, t697: f64, t3254: f64, t11789: f64, t11937: f64, t225: f64, t42051: f64, t366: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t42190, t42193, t42195, t42204, t42216, t42227) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1488(t11643, t11994, t12025, t3127, t3172, t3105, t3196, t11656, t2852, t3154, t2251, t11648, t3124);
        let t42246 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1489(t1041, t11622, t3172, t12021, t3173, t1032, t1040, t11902, t11762, t3241, t1047, t11659, t11703, t11705, t11714, t11883, t3177, t3238, t3248, t3255, t42216, t42227, t4892, t4899);
        let (t42249, t42251, t42254, t42257, t42259, t42261, t42262) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1490(t11752, t3241, t11755, t1011, t3247, t697, t3254, t11789, t11937, t225, t42051, t366);
    (t42190, t42193, t42195, t42204, t42246, t42249, t42251, t42254, t42257, t42259, t42261, t42262)
}
