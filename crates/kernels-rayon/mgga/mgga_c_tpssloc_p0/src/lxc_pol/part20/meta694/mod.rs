//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta694 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2643;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2644;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2645;
use chunk3::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2646;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta694(t28: f64, t3673: f64, t584: f64, t1081: f64, t3231: f64, t16: f64, t5181: f64, t591: f64, t11122: f64, t12000: f64, t12001: f64, t1302: f64, t16003: f64, t16006: f64, t1649: f64, t2: f64, t3711: f64, t39877: f64, t5178: f64, zeta_threshold: f64, t53827: f64, t16465: f64, t225: f64, t12344: f64, t5234: f64, t1369: f64, t16336: f64, t3876: f64, t16333: f64, t3866: f64, t1831: f64, t40284: f64, t12339: f64, t5314: f64, t40059: f64, t3872: f64, t12336: f64, t12361: f64, t1363: f64, t1367: f64, t16321: f64, t3783: f64, t40287: f64, t5240: f64, t820: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t53832, t53835, t53841, t53844, t53854) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2643(t28, t3673, t584, t1081, t3231, t16, t5181, t591, t11122, t12000, t12001, t1302, t16003, t16006, t1649, t2, t3711, t39877, t5178, zeta_threshold);
        let t53856 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2644(t53827, t53854);
        let (t53866, t53882, t53883, t53893, t53895, t53897) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2645(t16465, t225, t12344, t5234, t1369, t16336, t3876, t16333, t3866, t1831, t40284, t12339, t5314);
        let t53905 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2646(t1831, t40059, t16336, t3872, t12336, t12361, t1363, t1367, t16321, t16333, t3783, t40287, t5240, t5314, t53856, t53882, t53883, t53893, t53895, t53897, t820);
    (t53832, t53835, t53841, t53844, t53856, t53866, t53905)
}
