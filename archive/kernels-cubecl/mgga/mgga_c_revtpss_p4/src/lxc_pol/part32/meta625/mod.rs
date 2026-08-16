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

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

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
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta625<F: Float>(t28780: F, t97700: F, t6861: F, t7506: F, t1364: F, t30248: F, t786: F, t102329: F, t102339: F, t102346: F, t102661: F, t108206: F, t1444: F, t2097: F, t22252: F, t25930: F, t26079: F, t26304: F, t27837: F, t27864: F, t28863: F, t30071: F, t30247: F, t4003: F, t543: F, t7295: F, t7296: F, t7301: F, t7532: F, t94823: F, t96380: F, t96382: F, t108379: F, t7515: F, t102361: F, t102363: F, t102364: F, t102367: F, t108282: F, t22386: F, t22395: F, t25921: F, t25924: F, t27868: F, t28850: F, t28911: F, t28918: F, t30105: F, t30227: F, t30296: F, t30308: F, t7292: F, t7511: F, t75188: F, t7523: F, t96392: F, t97933: F, t30226: F, t689: F, t94768: F, t94763: F, t108279: F, t22453: F, t96463: F, t102372: F, t102378: F, t1903: F, t22415: F, t28815: F, t28888: F, t30267: F, t96398: F, t96401: F, t96403: F, t102386: F, t102396: F, t102404: F, t102409: F, t102411: F, t102422: F, t102594: F, t102656: F, t108259: F, t1904: F, t27972: F, t30257: F, t30309: F, t6918: F, t96410: F, t96412: F, t102434: F, t102439: F, t102453: F, t102458: F, t102462: F, t102465: F, t108225: F, t108448: F, t22433: F, t28912: F, t75012: F, t75267: F, t7528: F, t96456: F, t96460: F, t213: F, t6896: F, t7492: F, t102582: F, t102610: F, t102615: F, t102617: F, t102629: F, t1445: F, t30278: F, t8100: F, t94656: F, t96473: F, t96491: F, t96503: F, t96506: F, t96510: F, t96516: F, t98050: F, t3999: F, t8085: F, t102397: F, t102634: F, t102636: F, t14224: F, t14230: F, t22387: F, t26282: F, t28899: F, t30252: F, t5728: F, t75047: F, t75051: F, t75305: F, t94705: F, t96546: F, t96549: F, t102468: F, t108508: F, t108510: F, t108512: F, t108514: F, t108516: F, t108518: F, t108520: F, t108522: F, t108524: F, t108526: F, t108528: F, t102477: F, t102478: F, t108531: F, t108533: F, t108535: F, t108537: F, t108539: F, t108541: F, t108543: F, t108545: F, t108547: F, t108549: F, t102487: F, t102488: F, t102490: F, t102492: F, t108554: F, t108559: F, t108562: F, t94444: F, t94460: F, t98141: F, t98148: F, t98161: F, t102499: F, t102505: F, t102508: F, t102509: F, t108566: F, t108568: F, t108570: F, t108572: F, t108574: F, t108576: F, t108578: F, t98165: F, t98174: F, t102512: F, t102516: F, t102518: F, t108583: F, t108587: F, t96321: F, t96322: F, t96323: F, t98200: F, t98217: F, t98218: F, t98220: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t109573, t109598) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1978::<F>(t28780, t97700, t6861, t7506, t1364, t30248, t786, t102329, t102339, t102346, t102661, t108206, t1444, t2097, t22252, t25930, t26079, t26304, t27837, t27864, t28863, t30071, t30247, t4003, t543, t7295, t7296, t7301, t7532, t94823, t96380, t96382);
        let t109628 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1979::<F>(t108379, t7515, t102361, t102363, t102364, t102367, t108282, t1444, t2097, t22386, t22395, t25921, t25924, t25930, t27837, t27868, t28850, t28911, t28918, t30105, t30227, t30296, t30308, t7292, t7295, t7296, t7511, t75188, t7523, t96392, t97933);
        let t109656 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1980::<F>(t30226, t689, t94768, t94763, t108279, t7515, t22453, t96463, t102372, t102378, t109573, t1903, t22415, t25921, t27837, t28815, t28888, t30267, t543, t7295, t7296, t7301, t7511, t96398, t96401, t96403);
        let t109681 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1981::<F>(t102386, t102396, t102404, t102409, t102411, t102422, t102594, t102656, t108259, t1904, t25921, t25930, t26304, t27864, t27972, t30257, t30309, t6918, t7295, t7296, t7506, t96410, t96412);
        let t109704 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1982::<F>(t102434, t102439, t102453, t102458, t102462, t102465, t108225, t108282, t108448, t22433, t25930, t26304, t27868, t28911, t28912, t75012, t7511, t75267, t7528, t96456, t96460);
        let t109724 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1983::<F>(t213, t30247, t689, t6896, t7492, t102582, t102610, t102615, t102617, t102629, t1444, t1445, t30278, t7295, t8100, t94656, t96473, t96491, t96503, t96506, t96510, t96516, t98050);
        let t109756 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1984::<F>(t3999, t8085, t102397, t102634, t102636, t102661, t14224, t14230, t1903, t22387, t25930, t26282, t26304, t27868, t28899, t28911, t30252, t5728, t6896, t75047, t75051, t7511, t75305, t94705, t96546, t96549);
        let t109777 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1985::<F>(t102468, t108508, t108510, t108512, t108514, t108516, t108518, t108520, t108522, t108524, t108526, t108528);
        let t109788 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1986::<F>(t102477, t102478, t108531, t108533, t108535, t108537, t108539, t108541, t108543, t108545, t108547, t108549);
        let t109798 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1987::<F>(t102487, t102488, t102490, t102492, t108554, t108559, t108562, t94444, t94460, t98141, t98148, t98161);
        let t109808 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1988::<F>(t102499, t102505, t102508, t102509, t108566, t108568, t108570, t108572, t108574, t108576, t108578, t98165, t98174);
        let t109816 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1989::<F>(t102512, t102516, t102518, t108583, t108587, t96321, t96322, t96323, t98200, t98217, t98218, t98220);
    (t109598, t109628, t109656, t109681, t109704, t109724, t109756, t109777, t109788, t109798, t109808, t109816)
}
