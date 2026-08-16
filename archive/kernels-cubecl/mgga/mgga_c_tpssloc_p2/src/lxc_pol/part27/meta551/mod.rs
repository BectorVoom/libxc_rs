//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta551 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1986;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1987;
use chunk2::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1988;
use chunk3::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1989;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta551<F: Float>(t1824: F, t2006: F, t1352: F, t6914: F, t7737: F, t1351: F, t1834: F, t550: F, t6976: F, t1992: F, t3807: F, t5335: F, t22633: F, t5345: F, t1799: F, t562: F, t22705: F, t7736: F, t22704: F, t6883: F, t7741: F, t1332: F, t2013: F, t22693: F, t22707: F, t26379: F, t26381: F, t26386: F, t26390: F, t26393: F, t26398: F, t26401: F, t5230: F, t5344: F, t544: F, t7747: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let t26403 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1986::<F>(t1824, t2006);
        let (t26404, t26406, t26410, t26411, t26412, t26414, t26415, t26416) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1987::<F>(t1352, t26403, t6914, t7737, t1351, t1834, t550, t6976, t1992, t3807, t5335, t22633);
        let (t26418, t26419, t26421) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1988::<F>(t5345, t6976, t1992, t1799, t562);
        let (t26422, t26423, t26426, t26431) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1989::<F>(t1352, t26421, t6976, t22633, t22705, t7736, t22704, t6883, t7741, t1332, t2013, t22693, t22707, t26379, t26381, t26386, t26390, t26393, t26398, t26401, t26404, t26406, t26412, t26416, t26419, t5230, t5344, t544, t7747);
    (t26403, t26404, t26410, t26411, t26414, t26415, t26418, t26421, t26422, t26423, t26426, t26431)
}
