//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 876/1041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk876<F: Float>(t2178: F, t7407: F, t1882: F, t33188: F, t33057: F, t8392: F, t33230: F, t5: F, t22914: F, t34557: F, t135994: F, t135996: F, t137298: F, t137324: F, t1564: F, t22907: F, t22935: F, t25558: F, t25577: F, t25606: F, t25612: F, t25862: F, t3052: F, t32011: F, t32016: F, t32019: F, t32425: F, t5501: F, t6418: F, t7824: F, t925: F) -> (F, F, F, F, F) {
    let t140419 = t2178 * t7407;
    let t140424 = t1882 * t33188;
    let t140426 = t8392 * t33057;
    let t140469 = t5 * t33230;
    let t144308 = t22914 * t34557;
    let t144337 = -t144308 / 27.0 + 2.0 / 9.0 * t5501 * t22907 * t25862 - t137324 * t6418 / 18.0 + t5501 * t7824 * t137298 * t925 / 9.0 + 2.0 / 9.0 * t25577 * t7824 * t32019 * t3052 + t135994 / 54.0 + t32016 * t25606 / 9.0 + t32016 * t25612 / 9.0 - t25577 * t1564 * t32011 * t3052 / 9.0 + t22935 * t34557 / 9.0 - t25558 * t32425 / 9.0 + t135996 / 27.0;
    (t140419, t140424, t140426, t140469, t144337)
}
