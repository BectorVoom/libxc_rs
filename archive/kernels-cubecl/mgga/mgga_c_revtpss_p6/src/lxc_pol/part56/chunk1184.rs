//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 1184/1203 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk1184<F: Float>(t5412: F, t8937: F, t29127: F, t7642: F, t33494: F, t34969: F, t1042: F, t105236: F, t1122: F, t1203: F, t124645: F, t124675: F, t124711: F, t124819: F, t124838: F, t124893: F, t1250: F, t1294: F, t1795: F, t29111: F, t31993: F, t32015: F, t33414: F, t33424: F, t33425: F, t33446: F, t33464: F, t33525: F, t3367: F, t34929: F, t3626: F, t3719: F, t3720: F, t4181: F, t494: F, t5230: F, t5284: F, t8941: F) -> (F, F) {
    let t131657 = t8937 * t5412;
    let t131675 = t7642 * t29127;
    let t131683 = t34969 * t33494;
    let t131686 = F::cast_from(0.16940680055088280199e-2_f64) * t124838 * t33424 * t32015 * t124645 * t5230 - F::cast_from(0.11156198762715303246e-2_f64) * t124675 * t1042 * t1795 * t1250 * t1203 + F::cast_from(0.7437465841810202164e-3_f64) * t124893 * t1042 * t1795 * t1250 * t1294 + F::cast_from(0.57119737665102352616e0_f64) * t131657 * t8941 - F::cast_from(0.17347256376410398924e1_f64) * t33446 * t29111 + F::cast_from(0.11156198762715303246e-2_f64) * t124819 * t31993 * t3719 * t105236 + F::cast_from(0.28234466758480466999e-3_f64) * t33414 * t3720 * t494 * t5284 * t1250 - F::cast_from(0.37645955677973955999e-3_f64) * t124711 * t3626 * t34929 * t1122 + F::cast_from(0.17135921299530705785e1_f64) * t131675 * t33464 - F::cast_from(0.37645955677973955998e-3_f64) * t33425 * t3626 * t494 * t3367 * t4181 + F::cast_from(0.12395776403017003607e-3_f64) * t131683 * t33525;
    (t131657, t131686)
}
