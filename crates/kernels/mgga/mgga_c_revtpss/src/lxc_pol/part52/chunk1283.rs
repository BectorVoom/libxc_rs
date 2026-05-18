//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1283/1292 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1283<F: Float>(t1459: F, t34360: F, t7547: F, t7950: F, t111371: F, t1936: F, t572: F, t101705: F, t7953: F, t127453: F, t129014: F, t2040: F, t28978: F, t32377: F, t573: F, t5805: F, t7324: F, t7554: F, t7944: F, t8124: F, t8725: F) -> F {
    let t129018 = F::new(6.0) * t1459 * t34360;
    let t129026 = F::new(6.0) * t7547 * t7950;
    let t129029 = F::new(6.0) * t572 * t111371 * t1936;
    let t129032 = F::new(6.0) * t572 * t101705 * t1936;
    let t129034 = F::new(3.0) * t7547 * t7953;
    let t129037 = t129014 * t573 * param_d + F::new(6.0) * t2040 * t28978 + F::new(3.0) * t5805 * t8725 + F::new(6.0) * t7324 * t8124 + F::new(6.0) * t7554 * t7944 + t127453 + t129018 + t129026 + t129029 + t129032 + t129034 + t32377;
    t129037
}
