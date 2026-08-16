//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta545 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2232;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2233;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2234;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2235;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta545(t3450: f64, t5398: f64, t3449: f64, t18237: f64, t4908: f64, t3448: f64, t6138: f64, t3451: f64, t6144: f64, t18225: f64, t11583: f64, t5392: f64, t18221: f64, t15320: f64, t4904: f64, t15313: f64, t4919: f64, t11531: f64, t15265: f64, t15376: f64, t18404: f64, t3447: f64, t4901: f64, t15395: f64, t18206: f64, t15338: f64, t3431: f64, t6126: f64, t1174: f64, t6130: f64, t11539: f64, t6119: f64, t4889: f64, t4896: f64, t18215: f64, t4900: f64, t11570: f64, t11569: f64, t1180: f64, t15284: f64, t15287: f64, t15300: f64, t15307: f64, t18321: f64, t4937: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t18409, t18410, t18413, t18416, t18417, t18420, t18421, t18424, t18427) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2232(t3450, t5398, t3449, t18237, t4908, t3448, t6138, t3451, t6144, t18225, t11583, t5392);
        let t18442 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2233(t18427, t3449, t18221, t4908, t15320, t4904, t15313, t4919, t11531, t15265, t15376, t18404, t18410, t18413, t18417, t18421, t18424, t3447, t4901);
        let (t18443, t18447, t18452, t18455, t18458, t18460) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2234(t15395, t18206, t15338, t4904, t3447, t3431, t6126, t1174, t6130, t11539, t6119, t4889, t4896);
        let (t18469, t18473) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2235(t18215, t4900, t11570, t5392, t11569, t1180, t15284, t15287, t15300, t15307, t18321, t18443, t18447, t18452, t18455, t18458, t18460, t3447, t4889, t4937);
    (t18409, t18416, t18420, t18427, t18442, t18469, t18473)
}
