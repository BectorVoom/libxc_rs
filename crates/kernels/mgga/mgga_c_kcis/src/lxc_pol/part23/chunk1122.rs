//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1122/1323 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1122<F: Float>(t1616: F, t2104: F, t1307: F, t4440: F, t23096: F, t27636: F, t6176: F, t1615: F, t27614: F, t6183: F, t7984: F, t27583: F, t27607: F, t27653: F, t27654: F, t28410: F, t28413: F, t28415: F, t28430: F, t28451: F, t28454: F, t28701: F, t28714: F, t28727: F, t28772: F, t7968: F, t7978: F, t7981: F, t7986: F, t8213: F, t8222: F, t8226: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t28805 = t1616 * t2104;
    let t28806 = t28805 * t1307;
    let t28807 = t4440 * t28806;
    let t28810 = t27636 * t23096;
    let t28811 = t6176 * t28810;
    let t28814 = t2104 * t1615;
    let t28815 = t27614 * t28814;
    let t28816 = t6176 * t28815;
    let t28834 = t7984 * t6183;
    let t28835 = t6176 * t28834;
    let t28840 = -F::new(0.30952962962962962962e-2) * t28410 + F::new(0.11607361111111111111e-2) * t28413 + F::new(0.77382407407407407407e-3) * t28415 + F::new(0.34752604166666666667e-3) * t27607 * t8213 - F::new(0.17411041666666666666e-2) * t28430 + F::new(0.11584201388888888889e-3) * t27583 * t28807 - F::new(0.69505208333333333334e-3) * t7978 * t28811 - F::new(0.34752604166666666667e-3) * t7978 * t28816 - F::new(0.46377350260416666667e-4) * t7968 * t28816 + t27653 - F::new(0.11584201388888888889e-3) * t27654 + F::new(0.30891203703703703704e-3) * t28727 * t7981 - F::new(0.11584201388888888889e-3) * t27607 * t8222 - F::new(0.11607361111111111111e-2) * t28451 + F::new(0.77382407407407407407e-3) * t28454 + F::new(0.11584201388888888889e-3) * t27583 * t28701 + F::new(0.34752604166666666667e-3) * t7978 * t28772 + F::new(0.34752604166666666667e-3) * t27607 * t8226 + F::new(0.34752604166666666667e-3) * t7978 * t28835 + F::new(0.34752604166666666667e-3) * t28714 * t7986;
    (t28805, t28806, t28807, t28810, t28811, t28814, t28815, t28816, t28834, t28835, t28840)
}
