//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 638/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk638<F: Float>(t542: F, t8832: F, t1702: F, t2071: F, t1701: F, t39: F, t550: F, t133: F, t2037: F, t554: F, t2035: F, t1355: F, t139: F, t140: F, t1683: F, t1698: F, t1996: F, t2032: F, t2036: F, t2043: F, t399: F, t539: F, t540: F, t555: F, t7936: F, t8807: F, t8812: F, t8825: F, t8829: F, t8833: F, t8835: F) -> (F, F, F, F, F) {
    let t8838 = t542 * t8832;
    let t8847 = t1702 * t2071;
    let t8848 = t1701 * t8847;
    let t8851 = t550 * t39;
    let t8852 = t133 * t8851;
    let t8853 = t2037 * t554;
    let t8854 = t2035 * t8853;
    let t8859 = t542 * t8851;
    let t8864 = -F::cast_from(0.13867201135154723197e2_f64) * t2036 * t8807 * t139 + F::cast_from(0.43791161479435967991e1_f64) * t8812 * t2037 * t539 - F::cast_from(0.21895580739717983995e1_f64) * t2036 * t2037 * t555 + F::cast_from(0.28056686626142231644e2_f64) * t140 * t7936 - F::cast_from(0.17516464591774387197e2_f64) * t1996 * t1698 + F::cast_from(0.3624548033042297868e1_f64) * t2032 * t399 - F::cast_from(0.3624548033042297868e1_f64) * t8825 * t399 + F::cast_from(0.11477735437967276582e2_f64) * t1355 * t8829 + F::cast_from(0.3624548033042297868e1_f64) * t8833 * t8835 - F::cast_from(0.3624548033042297868e1_f64) * t8838 * t8835 - F::cast_from(0.11477735437967276582e2_f64) * t2043 * t8829 - F::cast_from(0.22955470875934553164e2_f64) * t540 * t1683 + F::cast_from(0.22955470875934553164e2_f64) * t1996 * t1683 - F::cast_from(0.1812274016521148934e1_f64) * t1355 * t8848 - F::cast_from(0.43791161479435967991e1_f64) * t8852 * t8854 + F::cast_from(0.1812274016521148934e1_f64) * t2043 * t8848 + F::cast_from(0.87582322958871935982e1_f64) * t8859 * t8854 + F::cast_from(0.87582322958871935983e1_f64) * t540 * t1698;
    (t8838, t8851, t8852, t8859, t8864)
}
