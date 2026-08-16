//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta102 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk566;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk567;
use chunk2::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk568;
use chunk3::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk569;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta102<F: Float>(t515: F, t518: F, t215: F, t2559: F, t535: F, t1314: F, t782: F, t2566: F, t795: F, t154: F, t557: F, t205: F, t792: F, t116: F, t534: F, t212: F, t2586: F, t2600: F, t541: F, t1337: F, t551: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t3704, t3711, t3725, t3726) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk566::<F>(t515, t518, t215, t2559, t535, t1314, t782);
        let (t3731, t3732) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk567::<F>(t2566, t535, t795, t154, t557);
        let t3733 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk568::<F>(t205, t3732);
        let (t3739, t3749, t3751, t3762, t3787) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk569::<F>(t1314, t792, t116, t534, t212, t2586, t2600, t541, t1337, t551);
    (t3704, t3711, t3725, t3726, t3731, t3732, t3733, t3739, t3749, t3751, t3762, t3787)
}
