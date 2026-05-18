//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 1109/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk1109<F: Float>(t10683: F, t35863: F, t446: F, t824: F, t143193: F, t3886: F, t28772: F, t6317: F, t152657: F, t24976: F, t24980: F, t152834: F, t152838: F, t152842: F, t152846: F, t152849: F, t152854: F, t152859: F, t152864: F, t152867: F, t152870: F, t152875: F, t152878: F, t152882: F) -> (F, F, F, F, F) {
    let t152886 = t446 * t10683 * t35863 * t824;
    let t152888 = t143193 * t3886;
    let t152890 = t6317 * t28772 * t152888;
    let t152893 = t24980 * t24976 * t152657;
    let t152895 = F::new(4.0) / F::new(3.0) * t152834 + F::new(4.0) / F::new(3.0) * t152838 - F::new(2.0) * t152842 - t152846 / F::new(9.0) - F::new(2.0) / F::new(9.0) * t152849 + t152854 / F::new(12.0) + t152859 / F::new(3.0) + t152864 / F::new(3.0) + t152867 / F::new(3.0) + t152870 / F::new(18.0) + t152875 / F::new(3.0) + t152878 / F::new(18.0) + F::new(4.0) / F::new(9.0) * t152882 - F::new(2.0) * t152886 + t152890 / F::new(27.0) + t152893 / F::new(18.0);
    (t152886, t152888, t152890, t152893, t152895)
}
