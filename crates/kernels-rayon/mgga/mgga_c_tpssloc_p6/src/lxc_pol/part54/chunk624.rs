//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 624/1484 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk624(t1174: f64, t1218: f64, t1227: f64, t1232: f64, t1737: f64, t3506: f64, t3515: f64, t3536: f64, t3577: f64, t488: f64, t4950: f64, t4954: f64, t4957: f64, t4959: f64, t4961: f64, t4966: f64, t4969: f64, t4974: f64, t4980: f64, t4984: f64, t4989: f64, t4994: f64, t4998: f64, t5002: f64, t5005: f64) -> f64 {
    let t5010 = -t3577 * t4950 / 4608.0_f64 - t3577 * t4954 / 4608.0_f64 + t4957 / 4608.0_f64 - t4959 / 864.0_f64 - t4961 * t488 / 576.0_f64 + t4966 * t488 / 3072.0_f64 - t1174 * t4969 / 144.0_f64 - t1227 * t4974 / 2304.0_f64 + t3506 * t4980 / 1536.0_f64 - t3515 * t4984 / 3072.0_f64 + 5.0_f64 / 13824.0_f64 * t1227 * t4989 - t4994 / 6912.0_f64 + t4998 / 4608.0_f64 + t5002 * t1218 / 3072.0_f64 - t5005 * t1232 / 4608.0_f64 + t3536 * t1737 / 3072.0_f64;
    t5010
}
