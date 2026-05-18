//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 987/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk987<F: Float>(t10514: F, t15256: F, t15260: F, t15263: F, t15267: F, t15271: F, t15273: F, t15274: F, t15277: F, t15281: F, t15286: F, t15291: F, t15296: F, t15300: F, t15304: F, t1901: F, t3281: F, t446: F) -> F {
    let t15307 = -F::new(4.0) / F::new(9.0) * t1901 * t15256 - F::new(2.0) / F::new(9.0) * t1901 * t15260 + F::new(2.0) / F::new(3.0) * t446 * t15263 + F::new(2.0) / F::new(9.0) * t3281 * t15267 + t15271 + t15273 - t446 * t15274 / F::new(3.0) + F::new(4.0) / F::new(3.0) * t446 * t15277 + F::new(2.0) / F::new(3.0) * t446 * t15281 + F::new(2.0) / F::new(3.0) * t446 * t15286 + F::new(8.0) / F::new(27.0) * t10514 + F::new(4.0) / F::new(27.0) * t1901 * t15291 + F::new(4.0) / F::new(27.0) * t1901 * t15296 - F::new(4.0) / F::new(9.0) * t1901 * t15300 - F::new(2.0) / F::new(9.0) * t1901 * t15304;
    t15307
}
