//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta593 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;
mod chunk8;
mod chunk9;
mod chunk10;
mod chunk11;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1977;
use chunk1::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1978;
use chunk2::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1979;
use chunk3::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1980;
use chunk4::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1981;
use chunk5::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1982;
use chunk6::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1983;
use chunk7::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1984;
use chunk8::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1985;
use chunk9::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1986;
use chunk10::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1987;
use chunk11::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1988;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta593(t98141: f64, t98144: f64, t98146: f64, t98148: f64, t98152: f64, t98156: f64, t94424: f64, t94430: f64, t94444: f64, t94449: f64, t98135: f64, t98154: f64, t98161: f64, t98165: f64, t98168: f64, t98180: f64, t94456: f64, t94460: f64, t98170: f64, t98172: f64, t98174: f64, t98176: f64, t98178: f64, t98182: f64, t98185: f64, t98187: f64, t98193: f64, t98200: f64, t98202: f64, t98206: f64, t94468: f64, t96321: f64, t96322: f64, t98189: f64, t98191: f64, t98197: f64, t98204: f64, t98218: f64, t98220: f64, t98222: f64, t98224: f64, t98226: f64, t98229: f64, t94479: f64, t96323: f64, t98211: f64, t98213: f64, t98215: f64, t98231: f64, t98235: f64, t98238: f64, t98243: f64, t94485: f64, t94498: f64, t94501: f64, t94503: f64, t94505: f64, t94509: f64, t94511: f64, t96326: f64, t98245: f64, t98253: f64, t98258: f64, t98260: f64, t98269: f64, t94514: f64, t94520: f64, t94527: f64, t94530: f64, t94534: f64, t94537: f64, t94540: f64, t96341: f64, t96342: f64, t98281: f64, t98285: f64, t94542: f64, t94546: f64, t94548: f64, t94552: f64, t94554: f64, t94557: f64, t94559: f64, t94561: f64, t94565: f64, t96358: f64, t96359: f64, t102480: f64, t1904: f64, t2439: f64, t26358: f64, t102453: f64, t102458: f64, t102462: f64, t102465: f64, t14224: f64, t213: f64, t225: f64, t25921: f64, t25924: f64, t25930: f64, t26304: f64, t27868: f64, t28841: f64, t4077: f64, t49306: f64, t561: f64, t7295: f64, t8085: f64, t96392: f64, t96456: f64, t96458: f64, t96460: f64, t96464: f64, t97858: f64, t28888: f64, t10073: f64, t25937: f64, t7282: f64, t13743: f64, t1444: f64, t1445: f64, t28792: f64, t28830: f64, t28911: f64, t48074: f64, t7296: f64, t7511: f64, t96473: f64, t96486: f64, t96491: f64, t96500: f64, t96503: f64, t96506: f64, t96510: f64, t102235: f64, t25904: f64, t102215: f64, t25878: f64, t3999: f64, t7506: f64, t102385: f64, t94383: f64, t102394: f64, t26260: f64, t27836: f64, t14230: f64, t14269: f64, t25909: f64, t28008: f64, t28899: f64, t28912: f64, t4078: f64, t7532: f64, t8104: f64, t96516: f64, t96527: f64, t96542: f64, t96546: f64, t97855: f64, t1385: f64, t1903: f64, t25933: f64, t27864: f64, t27972: f64, t28915: f64, t48025: f64, t94705: f64, t94823: f64, t96549: f64, t96550: f64, t96552: f64, t96556: f64, t96559: f64, t96561: f64, t96564: f64, t96565: f64) -> (f64, f64, f64, f64, f64) {
        let t102493 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1977(t98141, t98144, t98146, t98148, t98152, t98156, t94424, t94430, t94444, t94449, t98135, t98154);
        let t102507 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1978(t98161, t98165, t98168, t98180, t94456, t94460, t98170, t98172, t98174, t98176, t98178, t98182);
        let t102519 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1979(t98185, t98187, t98193, t98200, t98202, t98206, t94468, t96321, t96322, t98189, t98191, t98197, t98204);
        let t102533 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1980(t98218, t98220, t98222, t98224, t98226, t98229, t94479, t96323, t98211, t98213, t98215, t98231);
        let t102546 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1981(t98235, t98238, t98243, t94485, t94498, t94501, t94503, t94505, t94509, t94511, t96326, t98245, t98253);
        let t102558 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1982(t98258, t98260, t98269, t94514, t94520, t94527, t94530, t94534, t94537, t94540, t96341, t96342);
        let t102570 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1983(t98281, t98285, t94542, t94546, t94548, t94552, t94554, t94557, t94559, t94561, t94565, t96358, t96359);
        let (t102573, t102584) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1984(t102480, t102493, t102507, t102519, t102533, t102546, t102558, t102570, t1904, t2439, t26358, t102453, t102458, t102462, t102465, t14224, t213, t225, t25921, t25924, t25930, t26304, t27868, t28841, t4077, t49306, t561, t7295, t8085, t96392, t96456, t96458, t96460, t96464, t97858);
        let t102612 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1985(t213, t28888, t10073, t25937, t7282, t8085, t13743, t1444, t1445, t25921, t27868, t28792, t28830, t28911, t48074, t7295, t7296, t7511, t96473, t96486, t96491, t96500, t96503, t96506, t96510);
        let (t102615, t102617, t102622, t102629, t102634, t102636) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1986(t102235, t25904, t102215, t25878, t3999, t7506, t102385, t94383, t102394, t10073, t26260, t27836);
        let t102642 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1987(t102615, t102617, t102622, t102629, t102634, t102636, t14230, t14269, t25909, t27868, t28008, t28899, t28912, t4078, t7511, t7532, t8104, t96516, t96527, t96542, t96546, t97855);
        let t102669 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1988(t1385, t8085, t1903, t26304, t25930, t25933, t27864, t27868, t27972, t28911, t28915, t48025, t94705, t94823, t96392, t96549, t96550, t96552, t96556, t96559, t96561, t96564, t96565);
    (t102573, t102584, t102612, t102642, t102669)
}
