//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 1686/2259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1686<F: Float>(t21754: F, t606: F, t4186: F, t4210: F, t2282: F, t5825: F, t18281: F, t60: F, t10379: F, t1480: F, t21733: F, t21736: F, t21742: F, t21745: F, t4211: F, t4214: F, t44: F, t56: F, t5835: F, t5838: F, t5843: F, t614: F, t620: F) -> F {
    let t21755 = t21754 * t606;
    let t21758 = t4210 * t4186;
    let t21761 = t2282 * t5825;
    let t21762 = t21761 * t606;
    let t21765 = t60 * t18281;
    let t21768 = -F::cast_from(20.0_f64) / F::cast_from(27.0_f64) * t614 * t5835 - F::cast_from(5.0_f64) / F::cast_from(108.0_f64) * t44 * t21733 + F::cast_from(5.0_f64) / F::cast_from(9.0_f64) * t44 * t21736 - F::cast_from(20.0_f64) / F::cast_from(9.0_f64) * t614 * t5838 + F::cast_from(5.0_f64) / F::cast_from(18.0_f64) * t44 * t21742 + F::cast_from(5.0_f64) / F::cast_from(6.0_f64) * t44 * t21745 - F::cast_from(220.0_f64) / F::cast_from(27.0_f64) * t5843 * t620 - F::cast_from(40.0_f64) / F::cast_from(27.0_f64) * t1480 * t4211 + F::cast_from(40.0_f64) / F::cast_from(9.0_f64) * t1480 * t4214 + F::cast_from(5.0_f64) / F::cast_from(108.0_f64) * t56 * t21755 + F::cast_from(5.0_f64) / F::cast_from(9.0_f64) * t56 * t21758 + F::cast_from(5.0_f64) / F::cast_from(18.0_f64) * t56 * t21762 - F::cast_from(5.0_f64) / F::cast_from(6.0_f64) * t56 * t21765 + t10379;
    t21768
}
