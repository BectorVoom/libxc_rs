//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 638/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk638(t542: f64, t8832: f64, t1702: f64, t2071: f64, t1701: f64, t39: f64, t550: f64, t133: f64, t2037: f64, t554: f64, t2035: f64, t1355: f64, t139: f64, t140: f64, t1683: f64, t1698: f64, t1996: f64, t2032: f64, t2036: f64, t2043: f64, t399: f64, t539: f64, t540: f64, t555: f64, t7936: f64, t8807: f64, t8812: f64, t8825: f64, t8829: f64, t8833: f64, t8835: f64) -> (f64, f64, f64, f64, f64) {
    let t8838 = t542 * t8832;
    let t8847 = t1702 * t2071;
    let t8848 = t1701 * t8847;
    let t8851 = t550 * t39;
    let t8852 = t133 * t8851;
    let t8853 = t2037 * t554;
    let t8854 = t2035 * t8853;
    let t8859 = t542 * t8851;
    let t8864 = -0.13867201135154723197e2_f64 * t2036 * t8807 * t139 + 0.43791161479435967991e1_f64 * t8812 * t2037 * t539 - 0.21895580739717983995e1_f64 * t2036 * t2037 * t555 + 0.28056686626142231644e2_f64 * t140 * t7936 - 0.17516464591774387197e2_f64 * t1996 * t1698 + 0.3624548033042297868e1_f64 * t2032 * t399 - 0.3624548033042297868e1_f64 * t8825 * t399 + 0.11477735437967276582e2_f64 * t1355 * t8829 + 0.3624548033042297868e1_f64 * t8833 * t8835 - 0.3624548033042297868e1_f64 * t8838 * t8835 - 0.11477735437967276582e2_f64 * t2043 * t8829 - 0.22955470875934553164e2_f64 * t540 * t1683 + 0.22955470875934553164e2_f64 * t1996 * t1683 - 0.1812274016521148934e1_f64 * t1355 * t8848 - 0.43791161479435967991e1_f64 * t8852 * t8854 + 0.1812274016521148934e1_f64 * t2043 * t8848 + 0.87582322958871935982e1_f64 * t8859 * t8854 + 0.87582322958871935983e1_f64 * t540 * t1698;
    (t8838, t8851, t8852, t8859, t8864)
}
