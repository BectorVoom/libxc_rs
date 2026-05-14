//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 888/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk888<F: Float>(t24485: F, t24500: F, t24517: F, t24524: F, t24628: F, t27765: F, t27769: F, t27773: F, t27778: F, t27783: F, t27790: F, t27792: F, t24544: F, t24642: F, t27799: F, t27803: F, t27808: F, t27811: F, t27817: F, t27823: F, t27826: F, t27830: F, t27834: F, t27839: F) -> (F, F) {
    let t28069 = t27765 / 27.0 - t27769 / 9.0 - t27773 / 36.0 - t27778 / 36.0 - t24628 + t24485 / 9.0 - t27783 / 3.0 - 2.0 / 9.0 * t24500 + t24517 / 18.0 - t24524 / 27.0 + t27790 / 18.0 - t27792 / 54.0;
    let t28082 = -t27799 / 6.0 + t27803 / 18.0 - t27808 / 9.0 - t24642 + t27811 / 9.0 - t24544 / 54.0 - t27817 / 6.0 - t27823 / 8.0 - 2.0 / 9.0 * t27826 + 2.0 / 3.0 * t27830 + 2.0 / 3.0 * t27834 + t27839 / 3.0;
    (t28069, t28082)
}
