//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 866/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk866<F: Float>(t10570: F, t35828: F, t1486: F, t193: F, t2781: F, t35833: F, t1234: F, t7611: F, t852: F, t6308: F, t33819: F, t33846: F, t35822: F, t35826: F, t35831: F, t35836: F, t35840: F, t35844: F) -> (F, F, F, F, F, F, F, F) {
    let t35846 = t10570 * t35828;
    let t35848 = t1486 * t193 * t35846;
    let t35849 = t2781 * t35833;
    let t35851 = t1486 * t193 * t35849;
    let t35853 = t7611 * t1234;
    let t35854 = t852 * t35853;
    let t35856 = t6308 * t193 * t35854;
    let t35858 = t35822 / F::cast_from(2.0_f64) + t33819 + F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t35826 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t35831 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t35836 - t35840 / F::cast_from(6.0_f64) - t33846 - t35844 / F::cast_from(9.0_f64) - t35848 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t35851 + t35856 / F::cast_from(12.0_f64);
    (t35846, t35848, t35849, t35851, t35853, t35854, t35856, t35858)
}
