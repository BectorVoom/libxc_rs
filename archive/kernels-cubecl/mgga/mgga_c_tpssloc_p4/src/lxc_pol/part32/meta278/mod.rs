//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta278 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1256;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1257;
use chunk2::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1258;
use chunk3::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1259;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta278<F: Float>(t1799: F, t6968: F, t6637: F, t6888: F, t5335: F, t550: F, t6976: F, t1992: F, t1834: F, t1998: F, t214: F, t1985: F, t1825: F, t6987: F, t553: F, t7722: F, t1336: F, t1814: F, t2013: F, t544: F, t6967: F, t6975: F, t1378: F, t1375: F, t1843: F, t2016: F, t5215: F, t5321: F, t568: F, t6885: F, t6900: F, t6958: F, t7693: F, t7698: F, t7702: F, t7704: F, t7723: F, t7729: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t7732, t7733, t7734, t7736, t7737, t7738, t7740, t7741, t7742) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1256::<F>(t1799, t6968, t6637, t6888, t5335, t550, t6976, t1992, t1834, t1998, t214, t1985);
        let (t7745, t7747, t7749) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1257::<F>(t1825, t6987, t553, t7722, t1336, t1814, t2013, t544, t6967, t6975, t7734, t7738, t7742);
        let t7750 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1258::<F>(t1378, t7749);
        let t7752 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1259::<F>(t1375, t1843, t2016, t5215, t5321, t568, t6885, t6900, t6958, t7693, t7698, t7702, t7704, t7723, t7729, t7750);
    (t7732, t7733, t7736, t7737, t7740, t7741, t7745, t7747, t7749, t7750, t7752)
}
