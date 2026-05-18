//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 1157/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk1157<F: Float>(t152834: F, t152838: F, t152842: F, t152846: F, t152849: F, t152854: F, t152859: F, t152864: F, t152867: F, t152870: F, t152875: F, t152878: F, t152882: F, t152886: F, t152890: F, t152893: F) -> F {
    let t154156 = F::new(4.0) * t152834 + F::new(4.0) * t152838 - F::new(6.0) * t152842 - t152846 / F::new(3.0) - F::new(2.0) / F::new(3.0) * t152849 + t152854 / F::new(4.0) + t152859 + t152864 + t152867 + t152870 / F::new(6.0) + t152875 + t152878 / F::new(6.0) + F::new(4.0) / F::new(3.0) * t152882 - F::new(6.0) * t152886 + t152890 / F::new(9.0) + t152893 / F::new(6.0);
    t154156
}
