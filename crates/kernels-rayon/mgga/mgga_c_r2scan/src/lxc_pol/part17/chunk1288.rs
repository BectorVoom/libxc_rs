//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1288/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1288(t1039: f64, t12227: f64, t354: f64, t44926: f64, t44928: f64, t44931: f64, t44933: f64, t44935: f64, t44937: f64, t44940: f64, t44942: f64, t44953: f64, t44957: f64, t44962: f64, t44964: f64, t44971: f64, t44979: f64, t44986: f64, t44988: f64, t44997: f64, t45006: f64, t45011: f64, t45015: f64, t45023: f64, t45026: f64, t45030: f64, t45034: f64, t45036: f64, t45040: f64, t45044: f64, t45054: f64, t45079: f64, t45101: f64) -> f64 {
    let t45109 = -t44926 - t44928 - t44931 - t44933 + t44935 - t44937 + t44940 - t44942 + t354 * (t44953 + t44957 + t44962 + t44964 + t44971 + t44979 + t44986 + t44988 + t44997 + t45006 + t45011 + t45015 + t45040 + t45054 + t45079 + t45101) - t45023 - t45026 + t45030 + 2.0_f64 * t1039 * t12227 + t45034 + t45036 + t45044;
    t45109
}
