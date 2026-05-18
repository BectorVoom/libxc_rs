//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 1013/1200 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk1013<F: Float>(t1450: F, t8594: F, t4147: F, t8598: F, t211: F, t9644: F, t11006: F, t256: F, t2410: F, t3335: F, t11198: F, t340: F) -> (F, F, F, F, F, F, F) {
    let t37956 = t8594 * t1450;
    let t37972 = t8598 * t4147;
    let t39643 = F::new(1.0) / t9644 / t211;
    let t41077 = F::new(1.0) / t11006 / t256;
    let t41153 = t2410 * t2410;
    let t41154 = F::new(1.0) / t41153;
    let t41936 = t3335 * t3335;
    let t41937 = F::new(1.0) / t41936;
    let t42058 = F::new(1.0) / t11198 / t340;
    (t37956, t37972, t39643, t41077, t41154, t41937, t42058)
}
