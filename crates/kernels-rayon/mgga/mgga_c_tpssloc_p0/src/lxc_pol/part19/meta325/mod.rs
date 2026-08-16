//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta325 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1154;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1155;
use chunk2::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1156;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta325(t12283: f64, t12404: f64, t12413: f64, t12267: f64, t3802: f64, t3734: f64, t3792: f64, t12279: f64, t16398: f64, t12409: f64, t3719: f64, t12167: f64, t1314: f64, t9569: f64, t1329: f64, t12189: f64, t3770: f64, t12303: f64, t12368: f64, t12371: f64, t12419: f64, t1352: f64, t16224: f64, t16401: f64, t3803: f64, t3805: f64, t3806: f64, t3809: f64, t5246: f64, t5248: f64, t12313: f64, t3726: f64, t2559: f64, t3732: f64, t3766: f64, t12214: f64, t782: f64, t12320: f64, t154: f64, t1995: f64, t205: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t39971, t39973, t39975, t39978, t39983, t39989, t39993, t40000) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1154(t12283, t12404, t12413, t12267, t3802, t3734, t3792, t12279, t16398, t12409, t3719, t12167);
        let (t40005, t40010) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1155(t1314, t9569, t1329, t12189, t3770, t12279, t12303, t12368, t12371, t12419, t1352, t16224, t16401, t3803, t3805, t3806, t3809, t39971, t39973, t39975, t39978, t39983, t39989, t39993, t40000, t5246, t5248);
        let (t40012, t40018, t40019, t40021, t40022, t40025, t40026) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1156(t12313, t3726, t2559, t3732, t3766, t12214, t782, t12320, t154, t1995, t205, t3734);
    (t40005, t40010, t40012, t40018, t40019, t40021, t40022, t40025, t40026)
}
