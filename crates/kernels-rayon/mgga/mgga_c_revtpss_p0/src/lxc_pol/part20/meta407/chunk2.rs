//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1508/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1508(t3160: f64, t42664: f64, t11874: f64, t16048: f64, t1042: f64, t11252: f64, t11634: f64, t11862: f64, t11877: f64, t2251: f64, t3075: f64, t3127: f64, t3157: f64, t3164: f64, t42643: f64, t42648: f64, t42656: f64, t42658: f64, t42660: f64, t42662: f64, t42665: f64, t42669: f64, t4801: f64) -> f64 {
    let t42672 = t42664 * t3160;
    let t42675 = t11874 * t16048;
    let t42678 = -0.51448821741683684368e-2_f64 * t42643 * t11862 + 0.27439371595564631662e-1_f64 * t42648 * t11252 + 0.17149607247227894789e-2_f64 * t3127 * t1042 * t4801 * t2251 * t3075 + 0.30488190661738479624e-2_f64 * t42656 - 0.18292914397043087775e-1_f64 * t42658 - 0.18292914397043087775e-1_f64 * t42660 + 0.91464571985215438872e-2_f64 * t42662 + 0.25724410870841842184e-2_f64 * t42665 * t3157 + 0.51448821741683684368e-2_f64 * t42669 * t11634 - 0.12862205435420921092e-2_f64 * t42672 * t3164 - 0.13719685797782315831e-1_f64 * t42675 * t11877;
    t42678
}
