//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 715/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk715<F: Float>(t11281: F, t419: F, t1725: F, t3092: F, t11255: F, t11260: F, t11265: F, t11267: F, t11271: F, t11275: F, t11278: F, t8074: F, t8079: F, t8086: F) -> (F, F, F) {
    let t11282 = t419 * t11281;
    let t11284 = t1725 * t3092;
    let t11286 = F::new(0.12768721675925925926e-1) * t11255 + F::new(0.2269994964609053498e-1) * t8074 + t8079 + F::new(0.62424861526748971195e-1) * t8086 - F::new(0.85124811172839506173e-2) * t11260 + t11265 + F::new(0.85124811172839506173e-2) * t11267 + F::new(0.19862455940329218107e-1) * t11271 + F::new(0.3404992446913580247e-1) * t11275 - F::new(0.12768721675925925926e-1) * t11278 - F::new(0.51074886703703703704e-1) * t11282 + F::new(0.68099848938271604939e-1) * t11284;
    (t11282, t11284, t11286)
}
