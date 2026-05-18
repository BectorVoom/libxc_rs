//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 622/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk622<F: Float>(t86: F, t112: F, t113: F, t1927: F, t1934: F, t5: F, t502: F, t505: F, t8598: F, t8608: F, t2235: F, t177: F, t2280: F) -> (F, F, F) {
    let t87 = F::new(10000000.0) <= t86;
    let t8613 = piecewise3::<f64>(t87, F::new(0.0), t5 * t8598 * t113 / F::new(4.0) + F::new(3.0) / F::new(4.0) * t5 * t1927 * t505 + F::new(3.0) / F::new(4.0) * t5 * t502 * t1934 + t5 * t112 * t8608 / F::new(4.0));
    let t8614 = t5 * t2235;
    let t8618 = F::new(1.0) / t2280 / t177;
    (t8613, t8614, t8618)
}
