//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta525 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2305;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2306;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2307;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta525(t1287: f64, t17170: f64, t487: f64, t3584: f64, t5486: f64, t16756: f64, t5480: f64, t1770: f64, t3781: f64, t1234: f64, t12709: f64, t12756: f64, t1285: f64, t1291: f64, t16697: f64, t16751: f64, t16757: f64, t16763: f64, t16768: f64, t16772: f64, t16776: f64, t3666: f64, t3670: f64, t3746: f64, t3760: f64, t3763: f64, t3784: f64, t5216: f64, t5326: f64, t5459: f64, t5463: f64, t5474: f64, t5478: f64, t5487: f64, t1248: f64, t5230: f64, t1284: f64, t1811: f64, t1209: f64, t13392: f64, t5268: f64, t1042: f64, t1263: f64, t3362: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t17172, t17175, t17178, t17183, t17186) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2305(t1287, t17170, t487, t3584, t5486, t16756, t5480, t1770, t3781, t1234, t12709, t12756, t1285, t1291, t16697, t16751, t16757, t16763, t16768, t16772, t16776, t3666, t3670, t3746, t3760, t3763, t3784, t5216, t5326, t5459, t5463, t5474, t5478, t5487);
        let (t17188, t17191, t17192) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2306(t1248, t1287, t5230, t1284, t1811, t1209);
        let (t17198, t17199, t17202) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2307(t13392, t5268, t1042, t1263, t3362);
    (t17172, t17175, t17178, t17183, t17186, t17188, t17191, t17192, t17198, t17199, t17202)
}
