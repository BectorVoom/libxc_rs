//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta525 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2305;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2306;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2307;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta525<F: Float>(t1287: F, t17170: F, t487: F, t3584: F, t5486: F, t16756: F, t5480: F, t1770: F, t3781: F, t1234: F, t12709: F, t12756: F, t1285: F, t1291: F, t16697: F, t16751: F, t16757: F, t16763: F, t16768: F, t16772: F, t16776: F, t3666: F, t3670: F, t3746: F, t3760: F, t3763: F, t3784: F, t5216: F, t5326: F, t5459: F, t5463: F, t5474: F, t5478: F, t5487: F, t1248: F, t5230: F, t1284: F, t1811: F, t1209: F, t13392: F, t5268: F, t1042: F, t1263: F, t3362: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t17172, t17175, t17178, t17183, t17186) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2305::<F>(t1287, t17170, t487, t3584, t5486, t16756, t5480, t1770, t3781, t1234, t12709, t12756, t1285, t1291, t16697, t16751, t16757, t16763, t16768, t16772, t16776, t3666, t3670, t3746, t3760, t3763, t3784, t5216, t5326, t5459, t5463, t5474, t5478, t5487);
        let (t17188, t17191, t17192) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2306::<F>(t1248, t1287, t5230, t1284, t1811, t1209);
        let (t17198, t17199, t17202) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2307::<F>(t13392, t5268, t1042, t1263, t3362);
    (t17172, t17175, t17178, t17183, t17186, t17188, t17191, t17192, t17198, t17199, t17202)
}
