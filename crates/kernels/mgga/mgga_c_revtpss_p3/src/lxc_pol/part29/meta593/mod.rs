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

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

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
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta593<F: Float>(t98141: F, t98144: F, t98146: F, t98148: F, t98152: F, t98156: F, t94424: F, t94430: F, t94444: F, t94449: F, t98135: F, t98154: F, t98161: F, t98165: F, t98168: F, t98180: F, t94456: F, t94460: F, t98170: F, t98172: F, t98174: F, t98176: F, t98178: F, t98182: F, t98185: F, t98187: F, t98193: F, t98200: F, t98202: F, t98206: F, t94468: F, t96321: F, t96322: F, t98189: F, t98191: F, t98197: F, t98204: F, t98218: F, t98220: F, t98222: F, t98224: F, t98226: F, t98229: F, t94479: F, t96323: F, t98211: F, t98213: F, t98215: F, t98231: F, t98235: F, t98238: F, t98243: F, t94485: F, t94498: F, t94501: F, t94503: F, t94505: F, t94509: F, t94511: F, t96326: F, t98245: F, t98253: F, t98258: F, t98260: F, t98269: F, t94514: F, t94520: F, t94527: F, t94530: F, t94534: F, t94537: F, t94540: F, t96341: F, t96342: F, t98281: F, t98285: F, t94542: F, t94546: F, t94548: F, t94552: F, t94554: F, t94557: F, t94559: F, t94561: F, t94565: F, t96358: F, t96359: F, t102480: F, t1904: F, t2439: F, t26358: F, t102453: F, t102458: F, t102462: F, t102465: F, t14224: F, t213: F, t225: F, t25921: F, t25924: F, t25930: F, t26304: F, t27868: F, t28841: F, t4077: F, t49306: F, t561: F, t7295: F, t8085: F, t96392: F, t96456: F, t96458: F, t96460: F, t96464: F, t97858: F, t28888: F, t10073: F, t25937: F, t7282: F, t13743: F, t1444: F, t1445: F, t28792: F, t28830: F, t28911: F, t48074: F, t7296: F, t7511: F, t96473: F, t96486: F, t96491: F, t96500: F, t96503: F, t96506: F, t96510: F, t102235: F, t25904: F, t102215: F, t25878: F, t3999: F, t7506: F, t102385: F, t94383: F, t102394: F, t26260: F, t27836: F, t14230: F, t14269: F, t25909: F, t28008: F, t28899: F, t28912: F, t4078: F, t7532: F, t8104: F, t96516: F, t96527: F, t96542: F, t96546: F, t97855: F, t1385: F, t1903: F, t25933: F, t27864: F, t27972: F, t28915: F, t48025: F, t94705: F, t94823: F, t96549: F, t96550: F, t96552: F, t96556: F, t96559: F, t96561: F, t96564: F, t96565: F) -> (F, F, F, F, F) {
        let t102493 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1977::<F>(t98141, t98144, t98146, t98148, t98152, t98156, t94424, t94430, t94444, t94449, t98135, t98154);
        let t102507 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1978::<F>(t98161, t98165, t98168, t98180, t94456, t94460, t98170, t98172, t98174, t98176, t98178, t98182);
        let t102519 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1979::<F>(t98185, t98187, t98193, t98200, t98202, t98206, t94468, t96321, t96322, t98189, t98191, t98197, t98204);
        let t102533 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1980::<F>(t98218, t98220, t98222, t98224, t98226, t98229, t94479, t96323, t98211, t98213, t98215, t98231);
        let t102546 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1981::<F>(t98235, t98238, t98243, t94485, t94498, t94501, t94503, t94505, t94509, t94511, t96326, t98245, t98253);
        let t102558 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1982::<F>(t98258, t98260, t98269, t94514, t94520, t94527, t94530, t94534, t94537, t94540, t96341, t96342);
        let t102570 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1983::<F>(t98281, t98285, t94542, t94546, t94548, t94552, t94554, t94557, t94559, t94561, t94565, t96358, t96359);
        let (t102573, t102584) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1984::<F>(t102480, t102493, t102507, t102519, t102533, t102546, t102558, t102570, t1904, t2439, t26358, t102453, t102458, t102462, t102465, t14224, t213, t225, t25921, t25924, t25930, t26304, t27868, t28841, t4077, t49306, t561, t7295, t8085, t96392, t96456, t96458, t96460, t96464, t97858);
        let t102612 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1985::<F>(t213, t28888, t10073, t25937, t7282, t8085, t13743, t1444, t1445, t25921, t27868, t28792, t28830, t28911, t48074, t7295, t7296, t7511, t96473, t96486, t96491, t96500, t96503, t96506, t96510);
        let (t102615, t102617, t102622, t102629, t102634, t102636) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1986::<F>(t102235, t25904, t102215, t25878, t3999, t7506, t102385, t94383, t102394, t10073, t26260, t27836);
        let t102642 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1987::<F>(t102615, t102617, t102622, t102629, t102634, t102636, t14230, t14269, t25909, t27868, t28008, t28899, t28912, t4078, t7511, t7532, t8104, t96516, t96527, t96542, t96546, t97855);
        let t102669 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1988::<F>(t1385, t8085, t1903, t26304, t25930, t25933, t27864, t27868, t27972, t28911, t28915, t48025, t94705, t94823, t96392, t96549, t96550, t96552, t96556, t96559, t96561, t96564, t96565);
    (t102573, t102584, t102612, t102642, t102669)
}
