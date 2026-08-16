//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta346 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1385;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1386;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta346<F: Float>(t2970: F, t4522: F, t973: F, t10254: F, t3961: F, t10236: F, t10189: F, t1597: F, t2990: F, t2986: F, t2987: F, t4540: F, t2989: F, t3966: F, t2960: F, t4506: F, t10224: F, t1592: F, t4528: F, t1599: F, t698: F, t135: F, t4542: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t13830, t13835, t13839, t13847, t13850, t13851) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1385::<F>(t2970, t4522, t973, t10254, t3961, t10236, t10189, t1597, t2990, t2986, t2987, t4540);
        let (t13861, t13893, t13896, t13907, t13909, t13913) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1386::<F>(t2989, t3966, t2960, t4506, t10224, t1592, t973, t4528, t1599, t698, t135, t4542);
    (t13830, t13835, t13839, t13847, t13850, t13851, t13861, t13893, t13896, t13907, t13909, t13913)
}
