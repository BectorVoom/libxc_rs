//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta483 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1968;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1969;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta483(t16048: f64, t5335: f64, t3793: f64, t1332: f64, t5333: f64, t5230: f64, t68: f64, t12240: f64, t1352: f64, t16040: f64, t12189: f64, t1804: f64, t12188: f64, t12190: f64, t12194: f64, t12196: f64, t12197: f64, t12200: f64, t12205: f64, t12209: f64, t12212: f64, t12228: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t16049, t16052, t16055, t16060) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1968(t16048, t5335, t3793, t1332, t5333, t5230, t68);
        let (t16065, t16068, t16080) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1969(t12240, t5335, t1352, t16040, t12189, t1804, t12188, t12190, t12194, t12196, t12197, t12200, t12205, t12209, t12212, t12228);
    (t16049, t16052, t16055, t16060, t16065, t16068, t16080)
}
