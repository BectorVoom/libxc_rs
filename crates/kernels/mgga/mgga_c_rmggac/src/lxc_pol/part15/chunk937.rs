//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 937/963 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk937<F: Float>(t2186: F, t9935: F, t1970: F, t1971: F, t236: F, t29439: F, t2289: F, t9090: F, t1356: F, t36701: F, t41829: F, t41883: F, t41885: F, t46050: F, t46758: F, t47629: F, t47634: F, t47639: F, t47644: F, t47646: F, t47653: F, t4965: F, t4985: F, t5928: F, t739: F, t8804: F, t8824: F, t9867: F) -> (F,) {
    let t47663 = t2186 * t9935;
    let t47667 = t1970 * t1971 * t236 * t29439;
    let t47669 = t9090 * t2289;
    let t47671 = 0.53205749866622299248e-5 * t47629 + 0.85129199786595678796e-5 * t47634 - 0.12769379967989351819e-4 * t47639 + 0.12769379967989351819e-4 * t47644 - t41829 - 0.59590439850616975155e-4 * t47646 + 0.79828278012425390428e-1 * t5928 * t8804 + 0.51077519871957407276e-4 * t47653 + 0.79828278012425390428e-1 * t4965 * t9867 + 0.11974241701863808564e0 * t4985 * t8824 - 0.11974241701863808564e0 * t1356 * t46050 - 0.59871208509319042821e-1 * t739 * t46758 - 0.59590439850616975155e-4 * t47663 - 0.42564599893297839398e-5 * t47667 - t36701 + t41883 + t41885 + 0.59590439850616975155e-4 * t47669;
    (t47671,)
}
