//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 911/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk911<F: Float>(t1174: F, t1218: F, t1227: F, t1232: F, t1737: F, t3506: F, t3515: F, t3536: F, t3577: F, t488: F, t4950: F, t4954: F, t4957: F, t4959: F, t4961: F, t4966: F, t4969: F, t4974: F, t4980: F, t4984: F, t4989: F, t4994: F, t4998: F, t5002: F, t5005: F) -> F {
    let t5010 = -t3577 * t4950 / F::cast_from(4608.0_f64) - t3577 * t4954 / F::cast_from(4608.0_f64) + t4957 / F::cast_from(4608.0_f64) - t4959 / F::cast_from(864.0_f64) - t4961 * t488 / F::cast_from(576.0_f64) + t4966 * t488 / F::cast_from(3072.0_f64) - t1174 * t4969 / F::cast_from(144.0_f64) - t1227 * t4974 / F::cast_from(2304.0_f64) + t3506 * t4980 / F::cast_from(1536.0_f64) - t3515 * t4984 / F::cast_from(3072.0_f64) + F::cast_from(5.0_f64) / F::cast_from(13824.0_f64) * t1227 * t4989 - t4994 / F::cast_from(6912.0_f64) + t4998 / F::cast_from(4608.0_f64) + t5002 * t1218 / F::cast_from(3072.0_f64) - t5005 * t1232 / F::cast_from(4608.0_f64) + t3536 * t1737 / F::cast_from(3072.0_f64);
    t5010
}
