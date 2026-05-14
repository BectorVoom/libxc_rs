//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 800/1208 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk800<F: Float>(t179: F, t568: F, t8962: F, t2600: F, t2639: F, t3410: F, t600: F, t164: F, t3401: F, t3396: F, t615: F, t616: F, t8817: F, t1769: F, t3453: F, t1733: F, t2592: F, t2645: F, t5279: F, t5289: F, t612: F, t6896: F, t6935: F, t8946: F, t8950: F, t8955: F, t8959: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t8964 = t179 * t8962 * t568;
    let t8967 = t2600 * t2639;
    let t8968 = t179 * t8967;
    let t8971 = t3410 * t600;
    let t8972 = t8971 * t164;
    let t8973 = t179 * t8972;
    let t8976 = t3401 * t600;
    let t8978 = t179 * t8976 * t164;
    let t8981 = t3396 * t600;
    let t8983 = t179 * t8981 * t164;
    let t8988 = t615 * t616 * t8817;
    let t8991 = t1769 * t3453;
    let t8993 = -0.80031500487063509015e-2 * t8946 + 0.42874018118069736972e-3 * t2592 * t8950 - 0.12862205435420921092e-2 * t6896 * t8955 + 0.12862205435420921092e-2 * t2592 * t8959 + 0.85748036236139473944e-3 * t1733 * t8964 - 0.42874018118069736972e-3 * t2645 * t8968 - 0.21437009059034868486e-3 * t2645 * t8973 - 0.42874018118069736972e-2 * t5279 * t8978 + 0.85748036236139473944e-3 * t1733 * t8983 + 0.80031500487063509015e-2 * t6935 - t5289 - 0.85748036236139473944e-3 * t612 * t8988 - 0.20007875121765877254e-1 * t8991;
    (t8964, t8967, t8968, t8971, t8972, t8973, t8976, t8978, t8981, t8983, t8988, t8991, t8993)
}
