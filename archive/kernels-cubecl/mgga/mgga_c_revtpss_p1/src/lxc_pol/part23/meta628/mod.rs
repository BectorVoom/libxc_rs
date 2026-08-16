//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta628 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2318;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2319;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2320;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta628<F: Float>(t24973: F, t3783: F, t1794: F, t3302: F, t471: F, t20800: F, t24834: F, t3769: F, t1287: F, t24770: F, t487: F, t1234: F, t12717: F, t12751: F, t12756: F, t1285: F, t17183: F, t17192: F, t17307: F, t1770: F, t17958: F, t24698: F, t24964: F, t24974: F, t24978: F, t24981: F, t24986: F, t24989: F, t3755: F, t3767: F, t3782: F, t490: F, t5326: F, t5463: F, t5478: F, t6714: F, t6717: F, t6723: F, t6738: F, t6741: F, t24961: F, t1277: F, t1211: F, t24616: F, t24633: F, t1210: F, t12628: F, t1274: F, t1813: F, t1829: F, t20756: F, t24892: F, t24900: F, t24906: F, t3567: F, t5220: F, t5225: F, t5251: F, t5417: F, t6564: F, t6580: F, t6588: F, t6697: F, t6703: F, t6745: F, t24881: F, t12587: F, t1300: F, t1832: F, t198: F, t20692: F, t24468: F, t24478: F, t24482: F, t24484: F, t24490: F, t24492: F, t24496: F, t24500: F, t24501: F, t24763: F, t24767: F, t336: F, t5023: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t24994, t24998, t24999, t25002, t25005, t25009, t25014) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2318::<F>(t24973, t3783, t1794, t3302, t471, t20800, t24834, t3769, t1287, t24770, t487, t1234, t12717, t12751, t12756, t1285, t17183, t17192, t17307, t1770, t17958, t24698, t24964, t24974, t24978, t24981, t24986, t24989, t3755, t3767, t3782, t490, t5326, t5463, t5478, t6714, t6717, t6723, t6738, t6741);
        let (t25015, t25016, t25019, t25022, t25025) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2319::<F>(t24961, t25014, t1277, t1211, t24616, t24633, t1210, t12628, t1274, t1770, t1813, t1829, t20756, t24892, t24900, t24906, t3567, t5220, t5225, t5251, t5417, t6564, t6580, t6588, t6697, t6703, t6745);
        let (t25026, t25030) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2320::<F>(t24881, t25025, t12587, t1300, t1832, t198, t20692, t24468, t24478, t24482, t24484, t24490, t24492, t24496, t24500, t24501, t24763, t24767, t336, t5023);
    (t24994, t24998, t24999, t25002, t25005, t25009, t25015, t25016, t25019, t25022, t25026, t25030)
}
