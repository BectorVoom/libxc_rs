//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1138/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1138<F: Float>(t27873: F, t2842: F, t26760: F, t4792: F, t1020: F, t2179: F, t4923: F, t303: F, t4773: F, t7726: F, t26715: F, t26718: F, t27808: F, t27838: F, t27843: F, t27847: F, t27849: F, t27852: F, t27857: F, t27860: F, t27865: F, t27868: F, t27871: F, t7687: F, t7696: F, t7703: F, t8034: F, t8042: F) -> (F, F, F, F, F, F, F, F) {
    let t27874 = t2842 * t27873;
    let t27876 = t26760 * t4792;
    let t27877 = t1020 * t27876;
    let t27879 = t4923 * t2179;
    let t27880 = t303 * t27879;
    let t27882 = t7726 * t4773;
    let t27883 = t303 * t27882;
    let t27889 = F::new(0.16581944444444444444e-2) * t27838 - F::new(0.13901041666666666667e-2) * t7703 * t27808 + F::new(0.16581944444444444444e-2) * t27843 + F::new(0.33163888888888888888e-2) * t27847 + F::new(0.11054629629629629629e-2) * t27849 - F::new(0.44218518518518518517e-2) * t27852 - F::new(0.18534722222222222222e-2) * t7696 * t8034 + F::new(0.23168402777777777778e-3) * t27857 + F::new(0.16581944444444444444e-2) * t27860 + F::new(0.69505208333333333333e-3) * t7687 * t8034 - F::new(0.16581944444444444444e-2) * t27865 + F::new(0.11054629629629629629e-2) * t27868 - F::new(0.33163888888888888888e-2) * t27871 + F::new(0.27636574074074074073e-2) * t27874 - F::new(0.16581944444444444444e-2) * t27877 + F::new(0.24872916666666666666e-2) * t27880 - F::new(0.24872916666666666666e-2) * t27883 - F::new(0.18534722222222222222e-2) * t7696 * t8042 + F::new(0.23168402777777777778e-3) * t26715 + F::new(0.23168402777777777778e-3) * t26718;
    (t27874, t27876, t27877, t27879, t27880, t27882, t27883, t27889)
}
