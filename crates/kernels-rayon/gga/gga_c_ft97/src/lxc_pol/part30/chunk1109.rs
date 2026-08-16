//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 1109/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk1109(t10683: f64, t35863: f64, t446: f64, t824: f64, t143193: f64, t3886: f64, t28772: f64, t6317: f64, t152657: f64, t24976: f64, t24980: f64, t152834: f64, t152838: f64, t152842: f64, t152846: f64, t152849: f64, t152854: f64, t152859: f64, t152864: f64, t152867: f64, t152870: f64, t152875: f64, t152878: f64, t152882: f64) -> (f64, f64, f64, f64, f64) {
    let t152886 = t446 * t10683 * t35863 * t824;
    let t152888 = t143193 * t3886;
    let t152890 = t6317 * t28772 * t152888;
    let t152893 = t24980 * t24976 * t152657;
    let t152895 = 4.0_f64 / 3.0_f64 * t152834 + 4.0_f64 / 3.0_f64 * t152838 - 2.0_f64 * t152842 - t152846 / 9.0_f64 - 2.0_f64 / 9.0_f64 * t152849 + t152854 / 12.0_f64 + t152859 / 3.0_f64 + t152864 / 3.0_f64 + t152867 / 3.0_f64 + t152870 / 18.0_f64 + t152875 / 3.0_f64 + t152878 / 18.0_f64 + 4.0_f64 / 9.0_f64 * t152882 - 2.0_f64 * t152886 + t152890 / 27.0_f64 + t152893 / 18.0_f64;
    (t152886, t152888, t152890, t152893, t152895)
}
