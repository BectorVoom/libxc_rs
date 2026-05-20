//! MGGA_C_REVTPSS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1367/1422 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part5_v3rho3_2_chunk1367<F: Float>(t21754: F, t606: F, t4186: F, t4210: F, t2282: F, t5825: F, t18281: F, t60: F, t10379: F, t1480: F, t21733: F, t21736: F, t21742: F, t21745: F, t4211: F, t4214: F, t44: F, t56: F, t5835: F, t5838: F, t5843: F, t614: F, t620: F) -> F {
    let t21755 = t21754 * t606;
    let t21758 = t4210 * t4186;
    let t21761 = t2282 * t5825;
    let t21762 = t21761 * t606;
    let t21765 = t60 * t18281;
    let t21768 = -F::new(20.0) / F::new(27.0) * t614 * t5835 - F::new(5.0) / F::new(108.0) * t44 * t21733 + F::new(5.0) / F::new(9.0) * t44 * t21736 - F::new(20.0) / F::new(9.0) * t614 * t5838 + F::new(5.0) / F::new(18.0) * t44 * t21742 + F::new(5.0) / F::new(6.0) * t44 * t21745 - F::new(220.0) / F::new(27.0) * t5843 * t620 - F::new(40.0) / F::new(27.0) * t1480 * t4211 + F::new(40.0) / F::new(9.0) * t1480 * t4214 + F::new(5.0) / F::new(108.0) * t56 * t21755 + F::new(5.0) / F::new(9.0) * t56 * t21758 + F::new(5.0) / F::new(18.0) * t56 * t21762 - F::new(5.0) / F::new(6.0) * t56 * t21765 + t10379;
    t21768
}
