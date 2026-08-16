//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 1184/1203 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk1184(t5412: f64, t8937: f64, t29127: f64, t7642: f64, t33494: f64, t34969: f64, t1042: f64, t105236: f64, t1122: f64, t1203: f64, t124645: f64, t124675: f64, t124711: f64, t124819: f64, t124838: f64, t124893: f64, t1250: f64, t1294: f64, t1795: f64, t29111: f64, t31993: f64, t32015: f64, t33414: f64, t33424: f64, t33425: f64, t33446: f64, t33464: f64, t33525: f64, t3367: f64, t34929: f64, t3626: f64, t3719: f64, t3720: f64, t4181: f64, t494: f64, t5230: f64, t5284: f64, t8941: f64) -> (f64, f64) {
    let t131657 = t8937 * t5412;
    let t131675 = t7642 * t29127;
    let t131683 = t34969 * t33494;
    let t131686 = 0.16940680055088280199e-2_f64 * t124838 * t33424 * t32015 * t124645 * t5230 - 0.11156198762715303246e-2_f64 * t124675 * t1042 * t1795 * t1250 * t1203 + 0.7437465841810202164e-3_f64 * t124893 * t1042 * t1795 * t1250 * t1294 + 0.57119737665102352616e0_f64 * t131657 * t8941 - 0.17347256376410398924e1_f64 * t33446 * t29111 + 0.11156198762715303246e-2_f64 * t124819 * t31993 * t3719 * t105236 + 0.28234466758480466999e-3_f64 * t33414 * t3720 * t494 * t5284 * t1250 - 0.37645955677973955999e-3_f64 * t124711 * t3626 * t34929 * t1122 + 0.17135921299530705785e1_f64 * t131675 * t33464 - 0.37645955677973955998e-3_f64 * t33425 * t3626 * t494 * t3367 * t4181 + 0.12395776403017003607e-3_f64 * t131683 * t33525;
    (t131657, t131686)
}
