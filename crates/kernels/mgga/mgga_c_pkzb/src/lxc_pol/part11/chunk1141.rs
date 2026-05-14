//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1141/1208 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1141<F: Float>(t10945: F, t10982: F, t11037: F, t11044: F, t18163: F, t2031: F, t2096: F, t2104: F, t2105: F, t21686: F, t21715: F, t25459: F, t25462: F, t26457: F, t26460: F, t26510: F, t2899: F, t2901: F, t2923: F, t2945: F, t2946: F, t2976: F, t29795: F, t29858: F, t29905: F, t29908: F, t29911: F, t29918: F, t29928: F, t29970: F, t30044: F, t30095: F, t30156: F, t30164: F, t30803: F, t3650: F, t5956: F, t655: F, t758: F, t759: F, t7664: F, t7665: F, t7695: F, t7700: F, t7707: F, t7736: F, t7742: F, t9161: F, t9269: F) -> (F,) {
    let t30807 = -0.11433071498151929859e-2 * t2096 * t10982 - 0.11433071498151929859e-2 * t18163 * t10945 + 0.13719685797782315831e-1 * t7707 * t11044 - 0.42874018118069736972e-3 * t29908 + t30095 - 0.85748036236139473947e-3 * t29911 + t29795 + 0.14291339372689912324e-3 * t30164 - 0.85748036236139473947e-3 * t29928 - 0.25724410870841842184e-2 * t2899 * t7700 * t2031 * t3650 * t655 + 0.38586616306262763276e-2 * t7736 * t21686 * t5956 * t3650 * t759 + t29905 - 0.85748036236139473944e-3 * t26510 - 0.10289764348336736874e-1 * t26457 + 0.51448821741683684368e-2 * t26460 + 0.38586616306262763276e-2 * t2104 * t7695 * t9269 + t30156 + t21715 + t29970 - 0.25724410870841842183e-2 * t25462 + t30044 + t29858 + 0.25724410870841842183e-2 * t25459 + 0.38586616306262763276e-2 * t2945 * t758 * t2946 * t9161 - 0.12862205435420921092e-2 * t2104 * t2105 * t2976 * t11037 - 0.38586616306262763276e-2 * t7742 * t29918 * t7665 * t2901 + 0.64311027177104605458e-3 * t7664 * t29918 * t7665 * t2923 + t30803;
    (t30807,)
}
