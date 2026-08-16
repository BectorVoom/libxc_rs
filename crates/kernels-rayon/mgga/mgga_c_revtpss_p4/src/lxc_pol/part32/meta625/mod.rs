//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta625 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1978;
use chunk1::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1979;
use chunk2::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1980;
use chunk3::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1981;
use chunk4::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1982;
use chunk5::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1983;
use chunk6::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1984;
use chunk7::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1985;
use chunk8::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1986;
use chunk9::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1987;
use chunk10::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1988;
use chunk11::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1989;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta625(t28780: f64, t97700: f64, t6861: f64, t7506: f64, t1364: f64, t30248: f64, t786: f64, t102329: f64, t102339: f64, t102346: f64, t102661: f64, t108206: f64, t1444: f64, t2097: f64, t22252: f64, t25930: f64, t26079: f64, t26304: f64, t27837: f64, t27864: f64, t28863: f64, t30071: f64, t30247: f64, t4003: f64, t543: f64, t7295: f64, t7296: f64, t7301: f64, t7532: f64, t94823: f64, t96380: f64, t96382: f64, t108379: f64, t7515: f64, t102361: f64, t102363: f64, t102364: f64, t102367: f64, t108282: f64, t22386: f64, t22395: f64, t25921: f64, t25924: f64, t27868: f64, t28850: f64, t28911: f64, t28918: f64, t30105: f64, t30227: f64, t30296: f64, t30308: f64, t7292: f64, t7511: f64, t75188: f64, t7523: f64, t96392: f64, t97933: f64, t30226: f64, t689: f64, t94768: f64, t94763: f64, t108279: f64, t22453: f64, t96463: f64, t102372: f64, t102378: f64, t1903: f64, t22415: f64, t28815: f64, t28888: f64, t30267: f64, t96398: f64, t96401: f64, t96403: f64, t102386: f64, t102396: f64, t102404: f64, t102409: f64, t102411: f64, t102422: f64, t102594: f64, t102656: f64, t108259: f64, t1904: f64, t27972: f64, t30257: f64, t30309: f64, t6918: f64, t96410: f64, t96412: f64, t102434: f64, t102439: f64, t102453: f64, t102458: f64, t102462: f64, t102465: f64, t108225: f64, t108448: f64, t22433: f64, t28912: f64, t75012: f64, t75267: f64, t7528: f64, t96456: f64, t96460: f64, t213: f64, t6896: f64, t7492: f64, t102582: f64, t102610: f64, t102615: f64, t102617: f64, t102629: f64, t1445: f64, t30278: f64, t8100: f64, t94656: f64, t96473: f64, t96491: f64, t96503: f64, t96506: f64, t96510: f64, t96516: f64, t98050: f64, t3999: f64, t8085: f64, t102397: f64, t102634: f64, t102636: f64, t14224: f64, t14230: f64, t22387: f64, t26282: f64, t28899: f64, t30252: f64, t5728: f64, t75047: f64, t75051: f64, t75305: f64, t94705: f64, t96546: f64, t96549: f64, t102468: f64, t108508: f64, t108510: f64, t108512: f64, t108514: f64, t108516: f64, t108518: f64, t108520: f64, t108522: f64, t108524: f64, t108526: f64, t108528: f64, t102477: f64, t102478: f64, t108531: f64, t108533: f64, t108535: f64, t108537: f64, t108539: f64, t108541: f64, t108543: f64, t108545: f64, t108547: f64, t108549: f64, t102487: f64, t102488: f64, t102490: f64, t102492: f64, t108554: f64, t108559: f64, t108562: f64, t94444: f64, t94460: f64, t98141: f64, t98148: f64, t98161: f64, t102499: f64, t102505: f64, t102508: f64, t102509: f64, t108566: f64, t108568: f64, t108570: f64, t108572: f64, t108574: f64, t108576: f64, t108578: f64, t98165: f64, t98174: f64, t102512: f64, t102516: f64, t102518: f64, t108583: f64, t108587: f64, t96321: f64, t96322: f64, t96323: f64, t98200: f64, t98217: f64, t98218: f64, t98220: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t109573, t109598) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1978(t28780, t97700, t6861, t7506, t1364, t30248, t786, t102329, t102339, t102346, t102661, t108206, t1444, t2097, t22252, t25930, t26079, t26304, t27837, t27864, t28863, t30071, t30247, t4003, t543, t7295, t7296, t7301, t7532, t94823, t96380, t96382);
        let t109628 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1979(t108379, t7515, t102361, t102363, t102364, t102367, t108282, t1444, t2097, t22386, t22395, t25921, t25924, t25930, t27837, t27868, t28850, t28911, t28918, t30105, t30227, t30296, t30308, t7292, t7295, t7296, t7511, t75188, t7523, t96392, t97933);
        let t109656 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1980(t30226, t689, t94768, t94763, t108279, t7515, t22453, t96463, t102372, t102378, t109573, t1903, t22415, t25921, t27837, t28815, t28888, t30267, t543, t7295, t7296, t7301, t7511, t96398, t96401, t96403);
        let t109681 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1981(t102386, t102396, t102404, t102409, t102411, t102422, t102594, t102656, t108259, t1904, t25921, t25930, t26304, t27864, t27972, t30257, t30309, t6918, t7295, t7296, t7506, t96410, t96412);
        let t109704 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1982(t102434, t102439, t102453, t102458, t102462, t102465, t108225, t108282, t108448, t22433, t25930, t26304, t27868, t28911, t28912, t75012, t7511, t75267, t7528, t96456, t96460);
        let t109724 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1983(t213, t30247, t689, t6896, t7492, t102582, t102610, t102615, t102617, t102629, t1444, t1445, t30278, t7295, t8100, t94656, t96473, t96491, t96503, t96506, t96510, t96516, t98050);
        let t109756 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1984(t3999, t8085, t102397, t102634, t102636, t102661, t14224, t14230, t1903, t22387, t25930, t26282, t26304, t27868, t28899, t28911, t30252, t5728, t6896, t75047, t75051, t7511, t75305, t94705, t96546, t96549);
        let t109777 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1985(t102468, t108508, t108510, t108512, t108514, t108516, t108518, t108520, t108522, t108524, t108526, t108528);
        let t109788 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1986(t102477, t102478, t108531, t108533, t108535, t108537, t108539, t108541, t108543, t108545, t108547, t108549);
        let t109798 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1987(t102487, t102488, t102490, t102492, t108554, t108559, t108562, t94444, t94460, t98141, t98148, t98161);
        let t109808 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1988(t102499, t102505, t102508, t102509, t108566, t108568, t108570, t108572, t108574, t108576, t108578, t98165, t98174);
        let t109816 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1989(t102512, t102516, t102518, t108583, t108587, t96321, t96322, t96323, t98200, t98217, t98218, t98220);
    (t109598, t109628, t109656, t109681, t109704, t109724, t109756, t109777, t109788, t109798, t109808, t109816)
}
