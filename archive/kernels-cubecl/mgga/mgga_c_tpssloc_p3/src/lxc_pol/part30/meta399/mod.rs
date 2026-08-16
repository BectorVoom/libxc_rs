//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta399 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1520;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1521;
use chunk2::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1522;
use chunk3::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1523;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta399<F: Float>(t1041: F, t17659: F, t4630: F, t4641: F, t248: F, t3101: F, t5873: F, t3130: F, t376: F, t5872: F, t1022: F, t10482: F, t4582: F, t1539: F, t4650: F, t3071: F, t5867: F, t884: F, t10390: F, t10480: F, t10904: F, t13995: F, t14000: F, t14027: F, t17643: F, t17649: F, t17656: F, t3070: F, t4575: F, t5875: F, t5909: F, t5392: F, t607: F, t14172: F, t1409: F, t3966: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t17660, t17662, t17667, t17668, t17670, t17671) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1520::<F>(t1041, t17659, t4630, t4641, t248, t3101, t5873, t3130, t376, t5872, t1022, t10482);
        let (t17673, t17677, t17681, t17684) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1521::<F>(t17670, t17671, t4582, t1539, t4650, t3071, t5867, t884, t10390, t1041, t10480, t10904, t13995, t14000, t14027, t17643, t17649, t17656, t17660, t17662, t17668, t3070, t4575, t5875, t5909);
        let t17686 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1522::<F>(t5392, t607);
        let (t17688, t17691) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1523::<F>(t14172, t17686, t4582, t1409, t3966);
    (t17667, t17670, t17671, t17673, t17677, t17681, t17684, t17686, t17688, t17691)
}
