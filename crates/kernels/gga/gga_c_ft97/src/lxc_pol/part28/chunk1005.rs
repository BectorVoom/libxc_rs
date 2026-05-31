//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 1005/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk1005<F: Float>(t7165: F, t984: F, t1286: F, t1310: F, t136072: F, t136075: F, t144503: F, t144551: F, t1564: F, t2: F, t22873: F, t22935: F, t25535: F, t25553: F, t25564: F, t25577: F, t26: F, t28: F, t3052: F, t32016: F, t32423: F, t34352: F, t34553: F, t34614: F, t34620: F, t379: F, t4: F, t5495: F, t5501: F, t5620: F, t7162: F, t7824: F) -> F {
    let t144562 = t7165 * t984;
    let t144569 = -F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t25577 * t1564 * t32423 * t3052 + t32016 * t25564 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t1286 * t28 * t22873 * t34352 - t7162 * t25535 / F::cast_from(3.0_f64) + t136072 / F::cast_from(9.0_f64) + t34614 * t5620 / F::cast_from(6.0_f64) + F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t136075 + (t144503 + t144551) * t2 * t4 * t26 * t1310 / F::cast_from(6.0_f64) + t7162 * t25553 / F::cast_from(6.0_f64) - t5495 * t34620 / F::cast_from(3.0_f64) + t5501 * t7824 * t144562 * t379 / F::cast_from(9.0_f64) - t22935 * t34553 / F::cast_from(18.0_f64);
    t144569
}
