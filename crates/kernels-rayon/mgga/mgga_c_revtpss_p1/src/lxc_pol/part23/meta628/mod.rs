//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta628 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2318;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2319;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2320;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta628(t24973: f64, t3783: f64, t1794: f64, t3302: f64, t471: f64, t20800: f64, t24834: f64, t3769: f64, t1287: f64, t24770: f64, t487: f64, t1234: f64, t12717: f64, t12751: f64, t12756: f64, t1285: f64, t17183: f64, t17192: f64, t17307: f64, t1770: f64, t17958: f64, t24698: f64, t24964: f64, t24974: f64, t24978: f64, t24981: f64, t24986: f64, t24989: f64, t3755: f64, t3767: f64, t3782: f64, t490: f64, t5326: f64, t5463: f64, t5478: f64, t6714: f64, t6717: f64, t6723: f64, t6738: f64, t6741: f64, t24961: f64, t1277: f64, t1211: f64, t24616: f64, t24633: f64, t1210: f64, t12628: f64, t1274: f64, t1813: f64, t1829: f64, t20756: f64, t24892: f64, t24900: f64, t24906: f64, t3567: f64, t5220: f64, t5225: f64, t5251: f64, t5417: f64, t6564: f64, t6580: f64, t6588: f64, t6697: f64, t6703: f64, t6745: f64, t24881: f64, t12587: f64, t1300: f64, t1832: f64, t198: f64, t20692: f64, t24468: f64, t24478: f64, t24482: f64, t24484: f64, t24490: f64, t24492: f64, t24496: f64, t24500: f64, t24501: f64, t24763: f64, t24767: f64, t336: f64, t5023: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t24994, t24998, t24999, t25002, t25005, t25009, t25014) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2318(t24973, t3783, t1794, t3302, t471, t20800, t24834, t3769, t1287, t24770, t487, t1234, t12717, t12751, t12756, t1285, t17183, t17192, t17307, t1770, t17958, t24698, t24964, t24974, t24978, t24981, t24986, t24989, t3755, t3767, t3782, t490, t5326, t5463, t5478, t6714, t6717, t6723, t6738, t6741);
        let (t25015, t25016, t25019, t25022, t25025) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2319(t24961, t25014, t1277, t1211, t24616, t24633, t1210, t12628, t1274, t1770, t1813, t1829, t20756, t24892, t24900, t24906, t3567, t5220, t5225, t5251, t5417, t6564, t6580, t6588, t6697, t6703, t6745);
        let (t25026, t25030) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2320(t24881, t25025, t12587, t1300, t1832, t198, t20692, t24468, t24478, t24482, t24484, t24490, t24492, t24496, t24500, t24501, t24763, t24767, t336, t5023);
    (t24994, t24998, t24999, t25002, t25005, t25009, t25015, t25016, t25019, t25022, t25026, t25030)
}
